/*
    !TODO!: validation + optimizations (mmap instead of `fs::read()`, fast section find) + rewrite
*/

use crate::error::Result;
use goblin::elf::Elf;
use std::{
    fs,
    path::{Path, PathBuf},
};

const MODINFO_SECTION_NAME: &str = ".modinfo";
const DEBUG_LINE_SECTION_NAME: &str = ".debug_line_str";

#[derive(Debug, Clone)]
pub struct ModInfo {
    pub path: PathBuf,
    pub debug_line: String,
    retpoline: bool,
    license: String,
    name: String,
    author: Option<String>,
    depends: Option<String>, /* later -> validation to Option<Vec<String>> */
    version: Option<String>,
}

impl ModInfo {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let buf = fs::read(path.as_ref())?;
        let elf = Elf::parse(&buf)?;
        let mut raw_info_buf: &[u8] = &[];
        let mut raw_line_buf: &[u8] = &[];

        for section in elf.section_headers {
            let name = section.sh_name;
            let data = &buf[section.sh_offset as usize..][..section.sh_size as usize];
            if let Some(name_str) = elf.shdr_strtab.get_at(name) {
                match name_str {
                    MODINFO_SECTION_NAME => raw_info_buf = data,
                    DEBUG_LINE_SECTION_NAME => raw_line_buf = data,
                    _ => {}
                }
            }
        }
        /* validate -> [String; 6] */
        let modinfo = std::str::from_utf8(raw_info_buf).unwrap_or_default();

        let path = path.as_ref().to_path_buf();
        let debug_line = String::from_utf8_lossy(raw_line_buf)
            .into_owned()
            .replace("\0", " ")
            .trim()
            .into();

        Ok(Self {
            path,
            debug_line,
            /* temporary */
            retpoline: false,
            license: String::new(),
            name: String::new(),
            author: None,
            depends: None,
            version: None,
            /* temporary */
        })
    }

    fn validate(buf: &str) -> Result<[String; 6]> {
        assert!(!buf.is_empty());
        todo!("Implement validation");
    }
}

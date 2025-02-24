use goblin::elf::Elf;
use goblin::error::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Metadata {
    pub path: PathBuf,
    pub retpoline: bool,
    pub src_check_sum: u128,
    pub module_info: ModuleInfo,
}

struct ModuleInfo(String);

impl ModuleInfo {
    fn new(section_data: &[u8]) -> Self {
        ModuleInfo(
            std::str::from_utf8(section_data)
                .unwrap_or_default()
                .into(),
        )
    }
    
    pub fn license(&self) -> String {
        "hui".to_string()
    }
    
    // опшны так как в sextion_data может не быть поля
    pub fn author(&self) -> Option<String> {
        Some("hui".to_string())
    }
    pub fn desctiption(&self) -> Option<String> {
        Some("hui".to_string())
    }
    pub fn version(&self) -> Option<String> {
        Some("hui".to_string())
    }
}

impl Metadata {
    fn parse(str: String) -> () {
        ()
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mod_data = fs::read(path)?;
        let elf = Elf::parse(&mod_data)?;
        let section_data = String::new();

        for section in elf.section_headers {
            if elf.shdr_strtab.get_at(section.sh_name) == Some(".modinfo") {
                let data = &mod_data
                    [section.sh_offset as usize..(section.sh_offset + section.sh_size) as usize];
                section_data = std::string::String::from_utf8(data.to_vec()).unwrap_or_default();
            }
        }

        // parse section_data later - parse()

        Self { path }
    }
}

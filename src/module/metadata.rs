use goblin::{elf::Elf, error::Result};
use std::{fs, path::Path};

pub struct Metadata {
    pub comment_sec: Comment,
    pub mod_info_sec: ModInfo,
    pub debug_str_sec: DebugStr,
    pub kernel_notes_sec: KernelNotes,
    pub debug_line_str_sec: DebugLine,
}

impl Metadata {
    fn parse(&self) -> () {
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

#[doc = ".debug_line_str"]
#[repr(transparent)]
struct DebugLine(String);

impl DebugLine {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        DebugLine(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".debug_str"]
#[repr(transparent)]
struct DebugStr(String);

impl DebugStr {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        DebugStr(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".comment"]
#[repr(transparent)]
struct Comment(String);

impl Comment {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        Comment(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".note.Linux"]
#[repr(transparent)]
struct KernelNotes(String);

impl KernelNotes {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        KernelNotes(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".modinfo"]
struct ModInfo(String);

impl ModInfo {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        ModInfo(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

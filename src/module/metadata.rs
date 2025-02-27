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
    fn parse(&self) {}

    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mod_data = fs::read(path)?;
        let elf = Elf::parse(&mod_data)?;

        let mut modinfo_data: &[u8] = &[];
        let mut kernel_notes_data: &[u8] = &[];
        let mut comment_data: &[u8] = &[];
        let mut debug_str_data: &[u8] = &[];
        let mut debug_line_str_data: &[u8] = &[];

        elf.section_headers
            .iter()
            .filter_map(|section| {
                let name = elf.shdr_strtab.get_at(section.sh_name)?;
                let data = &mod_data
                    [section.sh_offset as usize..(section.sh_offset + section.sh_size) as usize];
                Some((name, data))
            })
            .for_each(|(name, data)| match name {
                ".modinfo" => modinfo_data = data,
                ".note.Linux" => kernel_notes_data = data,
                ".comment" => comment_data = data,
                ".debug_str" => debug_str_data = data,
                ".debug_line_str" => debug_line_str_data = data,
                _ => (), // until more sections
            });

        Ok(Self {
            comment_sec: Comment::new(comment_data),
            mod_info_sec: ModInfo::new(modinfo_data),
            debug_str_sec: DebugStr::new(debug_str_data),
            kernel_notes_sec: KernelNotes::new(kernel_notes_data),
            debug_line_str_sec: DebugLine::new(debug_line_str_data),
        })
    }
}

#[doc = ".debug_line_str"]
#[repr(transparent)]
pub struct DebugLine(String);

impl DebugLine {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        DebugLine(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".debug_str"]
#[repr(transparent)]
pub struct DebugStr(String);

impl DebugStr {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        DebugStr(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".comment"]
#[repr(transparent)]
pub struct Comment(String);

impl Comment {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        Comment(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".note.Linux"]
#[repr(transparent)]
pub struct KernelNotes(String);

impl KernelNotes {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        KernelNotes(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

#[doc = ".modinfo"]
#[repr(transparent)]
pub struct ModInfo(String);

impl ModInfo {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        ModInfo(std::str::from_utf8(section_data).unwrap_or_default().into())
    }
}

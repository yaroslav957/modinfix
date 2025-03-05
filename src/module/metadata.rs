/*
    !TODO!: fields validation + refactoring + optimizations
*/

use crate::error::Result;
use goblin::elf::Elf;
use std::{fs, path::Path};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ElfMetadata {
    pub comment_sec: Comment,
    pub mod_info_sec: ModInfo,
    pub debug_str_sec: DebugStr,
    pub kernel_notes_sec: KernelNotes,
    pub debug_line_str_sec: DebugLine,
}

impl ElfMetadata {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mod_data = fs::read(path)?;
        let elf = Elf::parse(&mod_data)?;
        let mut modinfo_data: &[u8] = &[];
        let mut kernel_notes_data: &[u8] = &[];
        let mut comment_data: &[u8] = &[];
        let mut debug_str_data: &[u8] = &[];
        let mut debug_line_str_data: &[u8] = &[];

        for section in &elf.section_headers {
            let Some(name) = elf.shdr_strtab.get_at(section.sh_name) else {
                continue;
            };

            let data = &mod_data
                [section.sh_offset as usize..(section.sh_offset + section.sh_size) as usize];

            match name {
                ".modinfo" => modinfo_data = data,
                ".note.Linux" => kernel_notes_data = data,
                ".comment" => comment_data = data,
                ".debug_str" => debug_str_data = data,
                ".debug_line_str" => debug_line_str_data = data,
                _ => (), // until more sections
            }
        }

        Ok(Self {
            comment_sec: Comment::new(comment_data),
            mod_info_sec: ModInfo::new(modinfo_data),
            debug_str_sec: DebugStr::new(debug_str_data),
            kernel_notes_sec: KernelNotes::new(kernel_notes_data),
            debug_line_str_sec: DebugLine::new(debug_line_str_data),
        })
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
/// .debug_line_str metadata section
pub struct DebugLine(String);

impl DebugLine {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        DebugLine(
            std::str::from_utf8(section_data)
                .unwrap_or_default()
                .replace('\0', " ")
                .trim()
                .into(),
        )
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
/// .debug_str section metadata
pub struct DebugStr(String);

impl DebugStr {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        DebugStr(
            std::str::from_utf8(section_data)
                .unwrap_or_default()
                .replace('\0', " ")
                .trim()
                .into(),
        )
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
/// .comment section metadata
pub struct Comment(String);

impl Comment {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        Comment(
            std::str::from_utf8(section_data)
                .unwrap_or_default()
                .replace('\0', " ")
                .trim()
                .into(),
        )
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
/// .note.Linux section metadata
pub struct KernelNotes(String);

impl KernelNotes {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        KernelNotes(
            std::str::from_utf8(section_data)
                .unwrap_or_default()
                .chars()
                .filter(|c| !c.is_control())
                .collect::<String>(),
        )
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
/// .modinfo section metadata
pub struct ModInfo(String);

impl ModInfo {
    pub(crate) fn new(section_data: &[u8]) -> Self {
        ModInfo(
            std::str::from_utf8(section_data)
                .unwrap_or_default()
                .replace('\0', " ")
                .trim()
                .into(),
        )
    }
}

/*
    !TODO!: fields validation (later) + optimizations (mmap instead of `fs::read()`)
*/

use crate::error::Result;
use goblin::elf::Elf;
use std::{fs, path::Path};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ElfMetadata {
    pub comment_section: Comment,
    pub modinfo_section: ModInfo,
    pub debugstr_section: DebugStr,
    pub kernelnotes_section: KernelNotes,
    pub debuglinestr_section: DebugLine,
}

use std::collections::HashMap;

impl ElfMetadata {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mod_data = fs::read(path)?;
        let elf = Elf::parse(&mod_data)?;
        let mut sections = HashMap::new();

        for section in &elf.section_headers {
            if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                let data = &mod_data
                    [section.sh_offset as usize..(section.sh_offset + section.sh_size) as usize];
                sections.insert(name, data);
            }
        }

        Ok(Self {
            comment_section: Comment::new(sections.get(".comment").copied().unwrap_or_default()),
            modinfo_section: ModInfo::new(sections.get(".modinfo").copied().unwrap_or_default()),
            debugstr_section: DebugStr::new(
                sections.get(".debug_str").copied().unwrap_or_default(),
            ),
            kernelnotes_section: KernelNotes::new(
                sections.get(".note.Linux").copied().unwrap_or_default(),
            ),
            debuglinestr_section: DebugLine::new(
                sections.get(".debug_line_str").copied().unwrap_or_default(),
            ),
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

/*
    !TODO!: validation (later) + optimizations (mmap instead of `fs::read()`)
*/

use crate::error::Result;
use goblin::elf::Elf;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ModInfo<'info> {
    pub path: PathBuf,
    pub retpoline: bool,
    pub license: &'info str,
    pub debug_line: &'info str,
    buff: [Option<&'info str>; 4], // name, author, depends, version
}

impl<'i> ModInfo<'i> {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mod_data = fs::read(path)?;
        let elf = Elf::parse(&mod_data)?;
        let mut modinfo_data: &[u8] = &[];
        todo!()
    }
}


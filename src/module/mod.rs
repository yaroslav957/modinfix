/*
    !TODO!: refactoring + optimizations + load/unload funcs
*/

use crate::{
    error::Result,
    module::{metadata::ElfMetadata, params::Params},
};
use std::{
    fs::File,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
};

pub mod error;
pub mod flags;
pub mod metadata;
pub mod params;

#[derive(Debug, Clone)]
pub struct Module {
    pub fd: i32,
    pub path: PathBuf,
    pub metadata: ElfMetadata,
    pub params: Params,
}

impl Module {
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            fd: File::open(&path)?.as_raw_fd(),
            metadata: ElfMetadata::new(&path)?,
            path: path.as_ref().to_path_buf(),
            params: Params::default(),
        })
    }
}

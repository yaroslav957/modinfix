use libc::{MODULE_INIT_IGNORE_MODVERSIONS, MODULE_INIT_IGNORE_VERMAGIC, O_NONBLOCK, O_TRUNC};
use std::{os::fd::RawFd, path::PathBuf};

use crate::module::{metadata::Metadata, params::Params};

pub mod metadata;
pub mod params;

pub struct Module {
    // cock_ed
    pub fd: RawFd,
    pub flags: (LoadFlag, UnloadFlag),
    pub path: PathBuf,
    // fifty-fifty cock_ed
    pub metadata: Metadata,
    // non-cock_ed
    pub params: Params,
}

#[repr(u32)]
pub enum LoadFlag {
    None = 0,
    IgnoreModuleVersion = MODULE_INIT_IGNORE_MODVERSIONS,
    IgnoreVersionMagic = MODULE_INIT_IGNORE_VERMAGIC,
    IgnoreAll = MODULE_INIT_IGNORE_MODVERSIONS | MODULE_INIT_IGNORE_VERMAGIC,
}

#[repr(i32)]
pub enum UnloadFlag {
    None = 0,
    Truncate = O_TRUNC,
    NonBlocking = O_NONBLOCK,
}

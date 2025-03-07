/*
    !TODO!: refactoring + optimizations + impl Drop for Module
*/

use crate::{
    error::{Error, Result},
    module::{
        error::ModuleError,
        flags::{LoadFlag, Syscall, UnloadFlag},
        metadata::ElfMetadata,
        params::Params,
    },
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

    pub fn fload(&self, flag: LoadFlag, params: Params) -> Result<()> {
        // SAFETY: `FLOAD=0x139`, Valid fd (checks on `::init()` step), const load flag and valid *const i8=CString ptr (NonNul on `::init()` step)
        let result =
            unsafe { libc::syscall(Syscall::FLoad as i64, self.fd, flag, params.0.as_ptr()) };

        if result < 0 {
            Err(Error::from(ModuleError::from(
                // SAFETY: Just trust me. (It's safe lol). TODO?: (rebase io::Error::last_os_error())
                unsafe { *libc::__errno_location() },
            )))
        } else {
            Ok(())
        }
    }

    pub fn unload(&self, flag: UnloadFlag) -> Result<()> {
        // SAFETY: `DELETE=0xB0`, Valid fd (checks on `::init()` step), const load flag
        let result = unsafe { libc::syscall(Syscall::Delete as i64, self.fd, flag) };

        if result < 0 {
            Err(Error::from(ModuleError::from(
                // SAFETY: Just trust me. (It's safe lol). TODO?: (rebase io::Error::last_os_error())
                unsafe { *libc::__errno_location() },
            )))
        } else {
            Ok(())
        }
    }
}

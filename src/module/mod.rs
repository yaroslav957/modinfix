/*
    !TODO!: refactoring + optimizations
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
    pub(crate) loaded: bool,
}

impl Module {
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            fd: File::open(&path)?.as_raw_fd(),
            metadata: ElfMetadata::new(&path)?,
            path: path.as_ref().to_path_buf(),
            params: Params::default(),
            loaded: false,
        })
    }

    pub fn fload(&mut self, flag: LoadFlag, params: Params) -> Result<()> {
        // SAFETY: `FLOAD=0x139`, Valid fd (checks on `::init()` step), const load flag and valid *const i8=CString ptr (NonNul on `::init()` step)
        let result =
            unsafe { libc::syscall(Syscall::FLoad as i64, self.fd, flag, params.0.as_ptr()) };

        if result < 0 {
            Err(Error::from(ModuleError::from(
                // SAFETY: Just FFI. Similar to `io::Error::last_os_error()`
                unsafe { *libc::__errno_location() },
            )))
        } else {
            self.loaded = true;
            Ok(())
        }
    }

    pub fn unload(&mut self, flag: UnloadFlag) -> Result<()> {
        if !self.loaded {
            return Err(Error::from(ModuleError::MODULE_NOT_FOUND));
        }

        // SAFETY: `DELETE=0xB0`, Valid fd (checks on `::init()` step), const load flag
        let result = unsafe { libc::syscall(Syscall::Delete as i64, self.fd, flag) };

        if result < 0 {
            Err(Error::from(ModuleError::from(
                // SAFETY: Just FFI. Similar to `io::Error::last_os_error()`
                unsafe { *libc::__errno_location() },
            )))
        } else {
            self.loaded = false;
            Ok(())
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if self.loaded {
            _ = self.unload(UnloadFlag::NONE);
        }
    }
}

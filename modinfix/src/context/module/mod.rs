/*
    !TODO!: refactoring + optimizations
*/

mod error;
pub use error::*;

mod flags;
pub use flags::*;

pub(crate) mod params;
use params::*;

use crate::error::{Error, Result};
use std::{fs::File, os::fd::AsRawFd};

pub(crate) fn load(buf: Vec<u8>, params: Params) -> Result<()> {
    // SAFETY: `LOAD=0xAF`, valid and successfully readed file (Vec<u8>).
    // Casting `buf.as_ptr=*const u8` to unit-type ptr to match the `init_module` signature.
    let result = unsafe {
        libc::syscall(
            Syscall::Load as i64,
            buf.as_ptr() as *const (),
            buf.len(),
            params.0.as_ptr(),
        )
    };

    if result < 0 {
        Err(Error::from(ModuleError::from(errno())))
    } else {
        Ok(())
    }
}

pub(crate) fn fload(file: File, flag: LoadFlag, params: Params) -> Result<()> {
    // SAFETY: `FLOAD=0x139`, Valid fd,
    // const load flag and valid *const i8=CString ptr (NonNul).
    let result = unsafe {
        libc::syscall(
            Syscall::FLoad as i64,
            file.as_raw_fd(),
            params.0.as_ptr(),
            flag,
        )
    };

    if result < 0 {
        Err(Error::from(ModuleError::from(errno())))
    } else {
        Ok(())
    }
}

pub(crate) fn unload(name: &str, flag: UnloadFlag) -> Result<()> {
    // SAFETY: `DELETE=0xB0`, const load flag
    let result = unsafe { libc::syscall(Syscall::Delete as i64, name.as_ptr(), flag) };

    if result < 0 {
        Err(Error::from(ModuleError::from(errno())))
    } else {
        Ok(())
    }
}

pub(crate) fn __is_loaded(_name: &str) -> bool {
    // Check if the module is loaded by querying the /sys/module directory
    // let path = format!("/sys/module/{name}");
    // let metadata = std::fs::metadata(&path);
    // metadata.is_ok()
    todo!()
}

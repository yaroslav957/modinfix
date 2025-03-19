/*
    !TODO!: refactoring + optimizations
*/

mod error;
pub use error::*;

mod flags;
pub use flags::*;

mod info;
pub use info::*;

mod params;
pub use params::*;

use crate::error::*;
use std::{fs::File, io::Read, os::fd::AsRawFd, path::Path};

// make refcount & size
#[derive(Debug)]
pub struct Module {
    pub file: File,
    pub modinfo: ModInfo,
    pub params: Params,
    pub(crate) loaded: bool,
}

impl Module {
    pub(crate) fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            file: File::open(path.as_ref())?,
            modinfo: ModInfo::new(path.as_ref())?,
            params: Params::default(),
            loaded: bool::default(),
        })
    }

    #[deprecated = "Starting with Kernel 3.8, it is preferable to use the `.fload(...)`"]
    pub fn load(&mut self, params: Params) -> Result<()> {
        if self.is_loaded() {
            return Err(Error::from(ModuleError::ALREADY_LOADED));
        }

        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf)?;
        /* SAFETY: `LOAD=0xAF`, valid and successfully readed file (file checks on `::init()` step).
        Casting `buf.as_ptr=*const u8` to unit-type ptr to match the `init_module` signature */
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
            self.loaded = true;
            Ok(())
        }
    }

    pub fn fload(&mut self, flag: LoadFlag, params: Params) -> Result<()> {
        if self.is_loaded() {
            return Err(Error::from(ModuleError::ALREADY_LOADED));
        }

        /* SAFETY: `FLOAD=0x139`, Valid fd (File checks on `::init()` step),
        const load flag and valid *const i8=CString ptr (NonNul on `::init()` step) */
        let result = unsafe {
            libc::syscall(
                Syscall::FLoad as i64,
                self.file.as_raw_fd(),
                params.0.as_ptr(),
                flag,
            )
        };

        if result < 0 {
            Err(Error::from(ModuleError::from(errno())))
        } else {
            self.loaded = true;
            Ok(())
        }
    }

    pub fn unload(&mut self, flag: UnloadFlag) -> Result<()> {
        if !self.is_loaded() {
            return Err(Error::from(ModuleError::MODULE_NOT_FOUND));
        }

        // SAFETY: `DELETE=0xB0`, Valid fd (File checks on `::init()` step), const load flag
        let result = unsafe { libc::syscall(Syscall::Delete as i64, self.file.as_raw_fd(), flag) };

        if result < 0 {
            Err(Error::from(ModuleError::from(errno())))
        } else {
            self.loaded = false;
            Ok(())
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded // rewrite after with /sys/module check after imfo validation
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if self.is_loaded() {
            _ = self.unload(UnloadFlag::NONE);
        }
    }
}

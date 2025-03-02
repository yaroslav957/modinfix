use libc::{EBADF, EBADMSG, EBUSY, EEXIST, EFAULT, EFBIG, EINVAL, ENOEXEC, ENOKEY, ENOMEM, EPERM};
use std::result;

#[derive(Debug)]
pub struct ModuleError(i32);

impl ModuleError {
    pub const INVALID_SIGNATURE: Self = Self(EBADMSG);
    pub const BUSY_TIMEOUT: Self = Self(EBUSY);
    pub const INVALID_ADDRESS: Self = Self(EFAULT);
    pub const BAD_CRYPTO_KEY: Self = Self(ENOKEY);
    pub const OUT_OF_MEMORY: Self = Self(ENOMEM);
    pub const PERMISSION_DENIED: Self = Self(EPERM);
    pub const ALREADY_LOADED: Self = Self(EEXIST);
    pub const INVALID_PARAMETERS: Self = Self(EINVAL);
    pub const INVALID_EXECUTABLE: Self = Self(ENOEXEC);
    pub const BAD_FILE_DESCRIPTOR: Self = Self(EBADF);
    pub const FILE_TOO_LARGE: Self = Self(EFBIG);
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Module(ModuleError),
    IO(std::io::Error),
}

pub type Result<T> = result::Result<T, Error>;

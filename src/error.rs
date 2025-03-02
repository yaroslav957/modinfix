use libc::{EBADF, EBADMSG, EBUSY, EEXIST, EFAULT, EFBIG, EINVAL, ENOEXEC, ENOKEY, ENOMEM, EPERM};
use std::{error, fmt, io, result};

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

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            EBADMSG => write!(f, "Invalid module signature format"),
            EBUSY => write!(f, "Module busy or timeout resolving symbols"),
            EFAULT => write!(f, "Invalid memory address"),
            ENOKEY => write!(f, "Missing or invalid cryptographic key"),
            ENOMEM => write!(f, "Out of memory"),
            EPERM => write!(f, "Permission denied (CAP_SYS_MODULE required)"),
            EEXIST => write!(f, "Module already loaded"),
            EINVAL => write!(f, "Invalid parameters or ELF structure"),
            ENOEXEC => write!(f, "Invalid ELF format or architecture mismatch"),
            EBADF => write!(f, "Bad file descriptor"),
            EFBIG => write!(f, "File too large"),
            _ => write!(f, "Unknown module error (OS code: {})", self.0),
        }
    }
}

impl From<ModuleError> for Error {
    fn from(e: ModuleError) -> Self {
        Error::Module(e)
    }
}

impl std::error::Error for ModuleError {} // <-- ? или нормальный импл сделать @Nano

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Module(ModuleError),
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Module(ref e) => write!(f, "[modinfix] Module error: {:?}", e),
            Error::IO(ref e) => write!(f, "[modinfix] IO error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Module(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

pub type Result<T> = result::Result<T, Error>;

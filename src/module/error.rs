use libc::{
    EBADF, EBADMSG, EBUSY, EEXIST, EFAULT, EFBIG, EINVAL, ENOENT, ENOEXEC, ENOKEY, ENOMEM, EPERM,
    EWOULDBLOCK,
};
use std::{error, fmt};

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
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
    pub const MODULE_NOT_FOUND: Self = Self(ENOENT);
    pub const DEPENDENCIES_BLOCK: Self = Self(EWOULDBLOCK);
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INVALID_SIGNATURE => write!(f, "Invalid module signature"),
            Self::BUSY_TIMEOUT => write!(f, "Module busy or timeout resolving symbols"),
            Self::INVALID_ADDRESS => write!(f, "Invalid memory address"),
            Self::BAD_CRYPTO_KEY => write!(f, "Missing or invalid crypto key"),
            Self::OUT_OF_MEMORY => write!(f, "Out of memory"),
            Self::PERMISSION_DENIED => write!(f, "Permission denied (CAP_SYS_MODULE required)"),
            Self::ALREADY_LOADED => write!(f, "Module already loaded"),
            Self::INVALID_PARAMETERS => write!(f, "Invalid parameters or ELF structure"),
            Self::INVALID_EXECUTABLE => write!(f, "Invalid ELF format or architecture mismatch"),
            Self::BAD_FILE_DESCRIPTOR => write!(f, "Bad file descriptor"),
            Self::FILE_TOO_LARGE => write!(f, "File too large"),
            Self::MODULE_NOT_FOUND => write!(f, "Module not found"),
            Self::DEPENDENCIES_BLOCK => write!(f, "Module has unresolved dependencies"),
            _ => write!(f, "Unknown module error (OS code: {})", self.0),
        }
    }
}

impl error::Error for ModuleError {}

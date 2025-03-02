use libc::{
    EBADF, EBADMSG, EBUSY, EEXIST, EFAULT, EFBIG, EINVAL, ENOENT, ENOEXEC, ENOKEY, ENOMEM, EPERM,
    EWOULDBLOCK,
};
use std::{error, fmt};

#[derive(Debug, PartialEq, Eq)]
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
        match self.0 {
            EBADMSG => write!(f, "Invalid module signature"),
            EBUSY => write!(f, "Module busy or timeout resolving symbols"),
            EFAULT => write!(f, "Invalid memory address"),
            ENOKEY => write!(f, "Missing or invalid crypto key"),
            ENOMEM => write!(f, "Out of memory"),
            EPERM => write!(f, "Permission denied (CAP_SYS_MODULE required)"),
            EEXIST => write!(f, "Module already loaded"),
            EINVAL => write!(f, "Invalid parameters or ELF structure"),
            ENOEXEC => write!(f, "Invalid ELF format or architecture mismatch"),
            EBADF => write!(f, "Bad file descriptor"),
            EFBIG => write!(f, "File too large"),
            ENOENT => write!(f, "Module not found"),
            EWOULDBLOCK => write!(f, "Module has unresolved dependencies"),
            _ => write!(f, "Unknown module error (OS code: {})", self.0),
        }
    }
}

impl error::Error for ModuleError {}

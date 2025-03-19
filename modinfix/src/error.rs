use crate::context::module::ModuleError;
use std::{error, fmt, io, result};

#[non_exhaustive]
pub enum Error {
    Module(ModuleError),
    IO(io::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Module(ref e) => write!(f, "[modinfix] Module error: {}", e),
            Error::IO(ref e) => write!(f, "[modinfix] IO error: {}", e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Module(ref e) => write!(f, "Module error: {}", e),
            Error::IO(ref e) => write!(f, "IO error: {}", e),
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

impl From<ModuleError> for Error {
    fn from(e: ModuleError) -> Self {
        Error::Module(e)
    }
}

impl From<goblin::error::Error> for Error {
    fn from(_: goblin::error::Error) -> Self {
        Error::Module(ModuleError::INVALID_EXECUTABLE)
    }
}

pub type Result<T> = result::Result<T, Error>;

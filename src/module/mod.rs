use std::path::PathBuf;

use crate::module::{metadata::Metadata, params::Params};

pub mod flags;
pub mod metadata;
pub mod params;

pub struct Module {
    pub fd: i32,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub params: Params,
}

impl Module {
    /*
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Self {
            fd: File::open(path.as_ref())?.as_raw_fd(),
            path: path.as_ref().to_path_buf(),
            metadata: Metadata::new(path).expect("Failed to create metadata"),
            params: Params::default(),
        })
    }
    */
}

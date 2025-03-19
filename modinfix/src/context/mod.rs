use module::Module;
use std::path::Path;

pub mod module;

pub struct Context;

/* Module impl */
impl Context {
    pub fn create_module<P: AsRef<Path>>(path: P) -> Result<Module, crate::error::Error> {
        Module::init(path.as_ref())
    }
}

/* kmesg impl */
impl Context {}

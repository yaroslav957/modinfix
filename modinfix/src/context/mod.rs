use crate::error::*;
use std::{fs::File, io::Read, path::Path};

pub mod module;
use module::{params::Params, *};

pub struct Context;

/* Module impl block */
impl Context {
    #[deprecated = "Starting with Kernel 3.8, it is preferable to use the `fload_module(...)`"]
    pub fn load_module<T, U>(path: T, params: U) -> Result<()>
    where
        T: AsRef<Path>,
        U: AsRef<str>,
    {
        let mut file = File::open(path.as_ref())?;
        let mut buf = Vec::default();
        let params = Params::new(params.as_ref())?;

        file.read_to_end(&mut buf)?;
        load(buf, params)
    }

    pub fn fload_module<T, U>(path: T, params: U, flag: LoadFlag) -> Result<()>
    where
        T: AsRef<Path>,
        U: AsRef<str>,
    {
        let file = File::open(path.as_ref())?;
        let params = Params::new(params.as_ref())?;

        fload(file, flag, params)
    }

    pub fn unload_module<T>(_path: T) -> Result<()>
    where
        T: AsRef<Path>,
    {
        todo!()
    }
}
/* Module impl block */

/* kmesg impl block */
impl Context {}
/* kmesg impl block */

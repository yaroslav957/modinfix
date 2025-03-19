/*
    !TODO!: delete shitty re + validation + refactoring
    (!!!!!! REWRITE !!!!!!)
*/

use std::{ffi::CString, io::Error};

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct Params(pub CString);

impl Params {
    pub fn new(params: &str) -> Result<Self, Error> {
        Ok(Self(CString::new(params)?))
    }
}

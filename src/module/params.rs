use std::{ffi::CString, io::Error};

#[repr(transparent)]
pub struct Params(CString);

impl Params {
    pub fn new(params: &[&str]) -> Result<Self, Error> {
        Ok(Self(CString::new(params.join(" "))?))
    }
}

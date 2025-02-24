use std::{
    collections::HashMap,
    ffi::CString,
    fmt::{Error, Write},
};

pub struct Params<'p>(HashMap<&'p str, &'p str>);

impl<'p> Params<'p> {
    pub fn new(params: HashMap<&'p str, &'p str>) -> Self {
        Params(params)
    }

    pub fn insert(&mut self, param: &'p str, value: &'p str) -> Option<&'p str> {
        self.0.insert(param, value)
    }

    pub fn get(&self, param: &str) -> Option<&&'p str> {
        self.0.get(param)
    }

    pub fn to_cstring(&self) -> Result<CString, Error> {
        let mut result = String::default();

        for (i, (key, value)) in self.0.iter().enumerate() {
            if i != 0 {
                result.push(' ');
            }
            write!(result, "{}={}", key, value)?;
        }

        Ok(CString::new(result).unwrap_or_default())
    }
}

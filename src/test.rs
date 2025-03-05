#[cfg(test)]
mod tests {
    use crate::{error::Result, module::Module};

    #[test]
    fn get_module_metadata() -> Result<()> {
        
        /* `Module::init()` is non-root operation,
            creating the mod instance with metadata included */
        let mod_instance = Module::init("..modules/mod.ko")?;
        let metadata = mod_instance.metadata.mod_info_sec;

        println!("{:?}", metadata);
        Ok(())
    }
}

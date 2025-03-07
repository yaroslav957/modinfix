#[cfg(test)]
mod tests {

    use crate::{
        error::Result,
        module::{Module, flags::LoadFlag, params::Params},
    };

    #[test]
    fn get_module_metadata() -> Result<()> {
        /*
        `Module::init(...)` is non-root operation,
        creating the mod instance with metadata included
        */
        let module = Module::init("../mod.ko")?;
        let metadata = module.metadata.comment_section;

        println!("{:?}", metadata);
        Ok(())
    }

    #[test]
    fn load_module() -> Result<()> {
        let module = Module::init("../mod.ko")?;
        /*
        `Module::fload(...)` is root operation,
        loading the mod instance into kernel space
        */
        let _ = module.fload(LoadFlag::NONE, Params::default())?;

        Ok(())
    }
}

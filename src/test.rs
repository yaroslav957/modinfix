#[cfg(test)]
mod tests {
    use crate::module::{Module, metadata::ElfMetadata};

    #[test]
    fn get_module_metadata() {
        /* `ElfMetadata::new()` is non-root operation, only extracts .ko metadata without creating a module instance */
        let _ = dbg!(ElfMetadata::new("..modules/mod.ko"));
    }

    #[test]
    fn get_module_instance() {
        /* `Module::init()` is non-root operation, creating the module instance with metadata included */
        let _ = dbg!(Module::init("..modules/mod.ko"));
    }
}

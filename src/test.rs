#[cfg(test)]
mod tests {
    use crate::module::{Module, metadata::ElfMetadata};

    #[test]
    fn get_metadata() {
        /* `ElfMetadata::new()` is non-root operation */
        let _ = dbg!(ElfMetadata::new("..modules/mod.ko"));
    }

    #[test]
    fn get_module_instance() {
        /* `Module::init()` is non-root operation */
        let _ = dbg!(Module::init("..modules/mod.ko"));
    }
}

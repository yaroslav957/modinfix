use bitflags::bitflags;
use libc::{MODULE_INIT_IGNORE_MODVERSIONS, MODULE_INIT_IGNORE_VERMAGIC, O_NONBLOCK, O_TRUNC};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    /// Flags controlling module loading behavior
    pub struct LoadFlag: u32 {
        /// No flags set (default value)
        const NONE = 0;
        /// Allows loading the module even if its version doesn't match the kernel's expectation
        #[doc(alias = "MODULE_INIT_IGNORE_MODVERSIONS")]
        const IGNORE_MODULE_VERSION = MODULE_INIT_IGNORE_MODVERSIONS;
        /// Allows loading modules compiled for different kernel versions
        #[doc(alias = "MODULE_INIT_IGNORE_VERMAGIC")]
        const IGNORE_VERSION_MAGIC = MODULE_INIT_IGNORE_VERMAGIC;
        /// Combines the effects of `IGNORE_MODULE_VERSION` and `IGNORE_VERSION_MAGIC`
        const IGNORE_ALL = Self::IGNORE_MODULE_VERSION.bits() | Self::IGNORE_VERSION_MAGIC.bits();
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    /// Flags controlling the behavior of module unloading
    pub struct UnloadFlag: i32 {
        /// No flags set (default value)
        const NONE = 0;
        /// Forces unloading of the module even if other modules depend on it
        #[doc(alias = "O_TRUNC")]
        const TRUNCATE = O_TRUNC;
        /// Does not block if the module is in use and returns an error instead
        #[doc(alias = "O_NONBLOCK")]
        const NON_BLOCKING = O_NONBLOCK;
        /// Combines the effects of `TRUNCATE` and `NON_BLOCKING`
        const BOTH = Self::TRUNCATE.bits() | Self::NON_BLOCKING.bits();
    }
}

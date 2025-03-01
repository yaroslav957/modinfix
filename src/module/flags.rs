use libc::{MODULE_INIT_IGNORE_MODVERSIONS, MODULE_INIT_IGNORE_VERMAGIC, O_NONBLOCK, O_TRUNC};

#[repr(u32)]
#[derive(Default)]
/// Flags controlling module loading behavior
pub enum LoadFlag {
    #[default]
    None = 0x0,
    /// Allows loading the module even if its version doesn't match the kernel's expectation
    IgnoreModuleVersion = MODULE_INIT_IGNORE_MODVERSIONS,
    /// Allows loading modules compiled for different kernel versions
    IgnoreVersionMagic = MODULE_INIT_IGNORE_VERMAGIC,
    /// Combines the effects of `IgnoreModuleVersion` and `IgnoreVersionMagic`
    IgnoreAll = MODULE_INIT_IGNORE_MODVERSIONS | MODULE_INIT_IGNORE_VERMAGIC,
}

#[repr(i32)]
#[derive(Default)]
/// Flags controlling the behavior of module unloading
pub enum UnloadFlag {
    #[default]
    /// No flags set
    None = 0x0,
    /// Forces unloading of the module even if other modules depend on it
    Truncate = O_TRUNC,
    /// Does not block if the module is in use and returns an error instead
    NonBlocking = O_NONBLOCK,
    /// Combines the effects of `Truncate` and `NonBlocking`
    Both = O_TRUNC | O_NONBLOCK,
}

use libc::{MODULE_INIT_IGNORE_MODVERSIONS, MODULE_INIT_IGNORE_VERMAGIC, O_NONBLOCK, O_TRUNC};

#[repr(u32)]
#[derive(Default)]
pub enum LoadFlag {
    #[default]
    None = 0x0,
    /// Allows load module even if it version does not match the kernel's expectation
    IgnoreModuleVersion = MODULE_INIT_IGNORE_MODVERSIONS,
    /// Allows load module compiled for different kernel versions
    IgnoreVersionMagic = MODULE_INIT_IGNORE_VERMAGIC,
    /// Combines the effects of `IgnoreModuleVersion` and `IgnoreVersionMagic`
    IgnoreAll = MODULE_INIT_IGNORE_MODVERSIONS | MODULE_INIT_IGNORE_VERMAGIC,
}

#[repr(i32)]
#[derive(Default)]
pub enum UnloadFlag {
    #[default]
    None = 0x0,
    /// The module is unloaded even if it lacks an exit function
    Truncate = O_TRUNC,
    /// If the module is in use, returns an error
    NonBlocking = O_NONBLOCK,
    /// The module is unloaded immediately, even if it is in use
    Both = O_TRUNC | O_NONBLOCK,
}

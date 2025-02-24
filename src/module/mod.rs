use libc::{MODULE_INIT_IGNORE_MODVERSIONS, MODULE_INIT_IGNORE_VERMAGIC};

use crate::module::{metadata::Metadata, params::Params};

pub mod metadata;
pub mod params;

pub struct Module<'m> {
    //cock_ed
    pub fd: i32,
    pub flag: Flag,
    // non-cock_ed
    pub params: Params<'m>, // <-- переделать это как врапер над стрингой
    pub metadata: Metadata,
}

#[repr(u32)]
pub enum Flag {
    None = 0,
    IgnoreModuleVersion = MODULE_INIT_IGNORE_MODVERSIONS,
    IgnoreVersionMagic = MODULE_INIT_IGNORE_VERMAGIC,
    Both = MODULE_INIT_IGNORE_MODVERSIONS | MODULE_INIT_IGNORE_VERMAGIC,
}

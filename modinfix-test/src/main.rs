use modinfix::{
    context::{Context as ctx, module::Params},
    error::Result,
};

#[allow(deprecated)]
fn main() -> Result<()> {
    let mut module = ctx::create_module("../mod.ko")?;
    module.load(Params::default())
}

mod embedded;
mod utils;

pub(crate) use boxed::boxed;
pub(crate) use diagnostics::*;
pub(crate) use runtime::error::E;
pub(crate) use runtime::*;
pub(crate) use utils::*;
pub(crate) use uuid::Uuid;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    embedded::register(efns)?;
    Ok(())
}

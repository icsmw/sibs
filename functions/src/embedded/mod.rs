mod console;
mod math;
mod status;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    console::register(efns)?;
    math::register(efns)?;
    status::register(efns)?;
    Ok(())
}

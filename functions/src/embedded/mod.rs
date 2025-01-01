mod console;
mod math;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    console::register(efns)?;
    math::register(efns)?;
    Ok(())
}

mod console;
mod debugging;
mod hash;
mod math;
mod signals;
mod status;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    console::register(efns)?;
    math::register(efns)?;
    status::register(efns)?;
    signals::register(efns)?;
    debugging::register(efns)?;
    hash::register(efns)?;
    Ok(())
}

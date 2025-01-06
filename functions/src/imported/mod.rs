mod fs;
mod strs;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    fs::register(efns)?;
    strs::register(efns)?;
    Ok(())
}

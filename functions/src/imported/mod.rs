mod fs;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    fs::register(efns)?;
    Ok(())
}

mod emit;
mod wait;
mod waiters;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    import_embedded_fn!(efns, wait);
    import_embedded_fn!(efns, emit);
    import_embedded_fn!(efns, waiters);
    Ok(())
}

mod executed;
mod is_cancelled;
mod is_failed;
mod is_success;
mod success;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    import_embedded_fn!(efns, is_success);
    import_embedded_fn!(efns, is_failed);
    import_embedded_fn!(efns, is_cancelled);
    import_embedded_fn!(efns, success);
    import_embedded_fn!(efns, executed);
    Ok(())
}

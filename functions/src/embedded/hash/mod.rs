mod inspect;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    import_embedded_fn!(efns, inspect);
    Ok(())
}

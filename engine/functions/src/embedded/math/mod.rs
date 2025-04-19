mod sum;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    import_embedded_fn!(efns, sum);
    Ok(())
}

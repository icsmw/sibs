mod exit;
mod print;

use crate::{executors::Store, executors::E};

pub fn register(store: &mut Store) -> Result<(), E> {
    store.insert(exit::name(), exit::execute)?;
    store.insert(print::name(), print::execute)?;
    Ok(())
}

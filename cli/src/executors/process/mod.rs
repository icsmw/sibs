mod abort;
mod exit;
mod print;
mod sleep;

use crate::{executors::Store, executors::E};

pub fn register(store: &mut Store) -> Result<(), E> {
    store.insert(abort::name(), abort::execute)?;
    store.insert(exit::name(), exit::execute)?;
    store.insert(print::name(), print::execute)?;
    store.insert(sleep::name(), sleep::execute)?;
    Ok(())
}

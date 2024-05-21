mod emit;
mod wait;

use crate::{executors::Store, executors::E};

pub fn register(store: &mut Store) -> Result<(), E> {
    store.insert(emit::name(), emit::execute)?;
    store.insert(wait::name(), wait::execute)?;
    Ok(())
}

mod emit;
mod wait;

use crate::{functions::Store, functions::E};

pub fn register(store: &mut Store) -> Result<(), E> {
    store.insert(emit::name(), emit::execute)?;
    store.insert(wait::name(), wait::execute)?;
    Ok(())
}

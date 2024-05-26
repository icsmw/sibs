mod emit;
mod wait;

use crate::{
    functions::{ExecutorFn, E},
    inf::Store,
};

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    store.insert(emit::name(), emit::execute)?;
    store.insert(wait::name(), wait::execute)?;
    Ok(())
}

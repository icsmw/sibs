mod abort;
mod exit;
mod print;
mod sleep;

use crate::{
    functions::{ExecutorFn, E},
    inf::Store,
};

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    store.insert(abort::name(), abort::execute)?;
    store.insert(exit::name(), exit::execute)?;
    store.insert(print::name(), print::execute)?;
    store.insert(sleep::name(), sleep::execute)?;
    Ok(())
}

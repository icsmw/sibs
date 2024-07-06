mod inspect;

use crate::{
    functions::{ExecutorFn, E},
    inf::Store,
};

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    store.insert(inspect::name(), inspect::execute)?;
    Ok(())
}

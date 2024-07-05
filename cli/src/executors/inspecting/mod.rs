mod hash;

use crate::{
    executors::{ExecutorFn, E},
    inf::Store,
};

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    store.insert(hash::name(), hash::execute)?;
    Ok(())
}

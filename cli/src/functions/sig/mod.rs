mod emit;
mod wait;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::{Store, ValueRef},
};

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    store.insert(
        emit::name(),
        ExecutorFnDescription::new(emit::execute, vec![ValueRef::String], ValueRef::Empty),
    )?;
    store.insert(
        wait::name(),
        ExecutorFnDescription::new(wait::execute, vec![ValueRef::String], ValueRef::Empty),
    )?;
    Ok(())
}

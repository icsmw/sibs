mod is_all_success;
mod is_any_fail;
mod is_fail;
mod is_success;
mod stop_on_fail;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::{Store, ValueRef},
};

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    store.insert(
        is_success::name(),
        ExecutorFnDescription::new(
            is_success::execute,
            vec![ValueRef::SpawnStatus],
            ValueRef::bool,
        ),
    )?;
    store.insert(
        is_fail::name(),
        ExecutorFnDescription::new(
            is_fail::execute,
            vec![ValueRef::SpawnStatus],
            ValueRef::bool,
        ),
    )?;
    store.insert(
        is_any_fail::name(),
        ExecutorFnDescription::new(
            is_any_fail::execute,
            vec![ValueRef::Vec(Box::new(ValueRef::SpawnStatus))],
            ValueRef::bool,
        ),
    )?;
    store.insert(
        is_all_success::name(),
        ExecutorFnDescription::new(
            is_all_success::execute,
            vec![ValueRef::Vec(Box::new(ValueRef::SpawnStatus))],
            ValueRef::bool,
        ),
    )?;
    store.insert(
        stop_on_fail::name(),
        ExecutorFnDescription::new(
            stop_on_fail::execute,
            vec![ValueRef::SpawnStatus],
            ValueRef::SpawnStatus,
        ),
    )?;
    Ok(())
}

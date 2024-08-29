mod inspect;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::{Store, ValueRef},
};

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    store.insert(
        inspect::name(),
        ExecutorFnDescription::new(
            inspect::execute,
            vec![
                ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf]),
                ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf]),
                ValueRef::bool,
            ],
            ValueRef::bool,
        ),
    )?;
    Ok(())
}

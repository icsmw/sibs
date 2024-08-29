mod get_cwd;
mod get_var;
mod set_cwd;
mod set_var;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::{Store, ValueRef},
};

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    store.insert(
        set_cwd::name(),
        ExecutorFnDescription::new(
            set_cwd::execute,
            vec![ValueRef::OneOf(vec![ValueRef::PathBuf, ValueRef::String])],
            ValueRef::PathBuf,
        ),
    )?;
    store.insert(
        get_cwd::name(),
        ExecutorFnDescription::new(get_cwd::execute, Vec::new(), ValueRef::PathBuf),
    )?;
    store.insert(
        set_var::name(),
        ExecutorFnDescription::new(
            set_var::execute,
            vec![
                ValueRef::String,
                ValueRef::OneOf(vec![
                    ValueRef::String,
                    ValueRef::Numeric,
                    ValueRef::bool,
                    ValueRef::PathBuf,
                ]),
            ],
            ValueRef::Empty,
        ),
    )?;
    store.insert(
        get_var::name(),
        ExecutorFnDescription::new(
            get_var::execute,
            vec![ValueRef::String],
            ValueRef::OneOf(vec![ValueRef::String, ValueRef::Empty]),
        ),
    )?;
    Ok(())
}

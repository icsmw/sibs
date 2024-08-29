mod abort;
mod exit;
mod print;
mod sleep;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::{Store, ValueRef},
};

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    store.insert(
        abort::name(),
        ExecutorFnDescription::new(
            abort::execute,
            vec![
                ValueRef::Optional(Box::new(ValueRef::Numeric)),
                ValueRef::Optional(Box::new(ValueRef::String)),
            ],
            ValueRef::Empty,
        ),
    )?;
    store.insert(
        exit::name(),
        ExecutorFnDescription::new(
            exit::execute,
            vec![
                ValueRef::Optional(Box::new(ValueRef::Numeric)),
                ValueRef::Optional(Box::new(ValueRef::String)),
            ],
            ValueRef::Empty,
        ),
    )?;
    store.insert(
        print::name(),
        ExecutorFnDescription::new(
            print::execute,
            vec![ValueRef::Repeated(Box::new(ValueRef::Any))],
            ValueRef::Empty,
        ),
    )?;
    store.insert(
        sleep::name(),
        ExecutorFnDescription::new(sleep::execute, vec![ValueRef::Numeric], ValueRef::Empty),
    )?;
    Ok(())
}

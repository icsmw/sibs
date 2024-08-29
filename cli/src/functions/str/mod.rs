use crate::{
    elements::FuncArg,
    functions::{ExecutorFnDescription, ExecutorPinnedResult, TryAnyTo, E},
    inf::{Context, Scope, Store, Value, ValueRef},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    #[import(str)]
    fn repeat(target: String, count: usize) -> Result<String, E> {
        Ok(target.repeat(count))
    }
    Ok(())
}

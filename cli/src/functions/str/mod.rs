use crate::{
    elements::FuncArg,
    functions::{ExecutorFn, ExecutorPinnedResult, TryAnyTo, E},
    inf::{Context, Scope, Store, Value},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    #[import(str)]
    fn repeat(target: String, count: usize) -> Result<String, E> {
        Ok(target.repeat(count))
    }
    Ok(())
}

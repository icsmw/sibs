use crate::{
    elements::FuncArg,
    functions::{ExecutorFn, ExecutorPinnedResult, TryAnyTo, E},
    inf::{AnyValue, Context, Scope, Store},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    #[import(str)]
    fn repeat(target: String, count: usize) -> Result<AnyValue, E> {
        Ok(AnyValue::String(target.repeat(count)))
    }
    Ok(())
}

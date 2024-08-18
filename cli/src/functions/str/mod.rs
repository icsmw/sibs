use crate::{
    elements::FuncArg,
    functions::{ExecutorFn, ExecutorPinnedResult, TryAnyTo, E},
    inf::{Value, Context, Scope, Store},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    #[import(str)]
    fn repeat(target: String, count: usize) -> Result<Value, E> {
        Ok(Value::String(target.repeat(count)))
    }
    Ok(())
}

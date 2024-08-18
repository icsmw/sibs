use crate::{
    elements::FuncArg,
    functions::{ExecutorFn, ExecutorPinnedResult, TryAnyTo, E},
    inf::{AnyValue, Context, Scope, Store},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    #[import(test)]
    fn err(msg: String) -> Result<AnyValue, E> {
        Err(E::FunctionExecuting(String::from("test::err"), msg))
    }

    #[import(test)]
    fn return_bool(value: bool) -> Result<AnyValue, E> {
        Ok(AnyValue::bool(value))
    }

    Ok(())
}

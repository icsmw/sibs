use crate::{
    elements::FuncArg,
    functions::{ExecutorFn, ExecutorPinnedResult, TryAnyTo, E},
    inf::{Context, Scope, Store, Value},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    #[import(test)]
    fn err(msg: String) -> Result<String, E> {
        Err(E::FunctionExecuting(String::from("test::err"), msg))
    }

    #[import(test)]
    fn return_bool(value: bool) -> Result<bool, E> {
        Ok(value)
    }

    Ok(())
}

use crate::{
    executors::Store,
    executors::{ExecutorPinnedResult, TryAnyTo, E},
    inf::{AnyValue, Context, Scope},
};
use importer::import;

pub fn register(store: &mut Store) -> Result<(), E> {
    #[import(test)]
    fn err(msg: String) -> Result<(), E> {
        Err(E::FunctionExecuting(String::from("test::err"), msg))
    }

    #[import(test)]
    fn return_bool(value: bool) -> Result<bool, E> {
        Ok(value)
    }

    Ok(())
}

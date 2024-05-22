use crate::{
    functions::Store,
    functions::{ExecutorPinnedResult, TryAnyTo, E},
    inf::{AnyValue, Context, Scope},
};
use importer::import;

pub fn register(store: &mut Store) -> Result<(), E> {
    #[import(str)]
    fn repeat(target: String, count: usize) -> Result<String, E> {
        Ok(target.repeat(count))
    }
    Ok(())
}

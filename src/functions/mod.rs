pub mod import;
pub mod os;

use crate::{
    error::E,
    inf::any::AnyValue,
    inf::context::Context,
    reader::{self, entry::Function},
};
use std::{future::Future, pin::Pin};

pub type ExecutorPinnedResult<'a> = Pin<Box<dyn Future<Output = ExecutorResult> + 'a>>;
pub type ExecutorResult = Result<Option<AnyValue>, E>;
pub type ExecutorFn = for<'a> fn(&'a Function, &'a mut Context) -> ExecutorPinnedResult<'a>;

pub trait Executor {
    fn execute<'a>(function: &'a Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a>;
    fn get_name() -> String;
}

pub fn register(cx: &mut Context) -> Result<(), reader::error::E> {
    cx.add_fn(
        import::Import::get_name(),
        <import::Import as Executor>::execute,
    )?;
    cx.add_fn(os::Os::get_name(), <os::Os as Executor>::execute)?;
    Ok(())
}

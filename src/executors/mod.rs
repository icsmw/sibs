mod error;
pub mod get_os;
pub mod import;
pub mod os;
pub mod repeat;

use crate::{entry::Function, inf::any::AnyValue, inf::context::Context};
pub use error::E;
use std::{future::Future, pin::Pin};

pub type ExecutorPinnedResult<'a> = Pin<Box<dyn Future<Output = ExecutorResult> + 'a>>;
pub type ExecutorResult = Result<Option<AnyValue>, E>;
pub type ExecutorFn = for<'a> fn(&'a Function, &'a mut Context) -> ExecutorPinnedResult<'a>;

pub trait Executor {
    fn execute<'a>(function: &'a Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a>;
    fn get_name() -> String;
}

pub fn register(cx: &mut Context) -> Result<(), E> {
    cx.add_fn(
        import::Import::get_name(),
        <import::Import as Executor>::execute,
    )?;
    cx.add_fn(os::Os::get_name(), <os::Os as Executor>::execute)?;
    cx.add_fn(
        repeat::Repeat::get_name(),
        <repeat::Repeat as Executor>::execute,
    )?;
    cx.add_fn(
        get_os::GetOs::get_name(),
        <get_os::GetOs as Executor>::execute,
    )?;
    Ok(())
}

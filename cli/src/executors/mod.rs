pub mod env;
mod error;
pub mod fs;
pub mod get_os;
pub mod import;
pub mod os;
pub mod repeat;

use crate::{inf::any::AnyValue, inf::context::Context};
pub use error::E;
use std::{future::Future, pin::Pin};

pub type ExecutorPinnedResult<'a> = Pin<Box<dyn Future<Output = ExecutorResult> + 'a>>;
pub type ExecutorResult = Result<AnyValue, E>;
pub type ExecutorFn = for<'a> fn(Vec<AnyValue>, &'a mut Context) -> ExecutorPinnedResult<'a>;

pub trait Executor {
    fn execute(args: Vec<AnyValue>, cx: &mut Context) -> ExecutorPinnedResult;
    fn get_name() -> String;
}

pub trait TryAnyTo<T> {
    fn try_to(&self) -> Result<T, E>;
}

pub fn register(cx: &mut Context) -> Result<(), E> {
    cx.functions().add(
        import::Import::get_name(),
        <import::Import as Executor>::execute,
    )?;
    cx.functions()
        .add(os::Os::get_name(), <os::Os as Executor>::execute)?;
    cx.functions().add(
        repeat::Repeat::get_name(),
        <repeat::Repeat as Executor>::execute,
    )?;
    cx.functions().add(
        get_os::GetOs::get_name(),
        <get_os::GetOs as Executor>::execute,
    )?;
    fs::register(cx)?;
    env::register(cx)?;
    Ok(())
}

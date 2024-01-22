pub mod import;
pub mod os;

use crate::{
    error::E,
    inf::any::AnyValue,
    inf::context::Context,
    reader::{self, entry::Function},
};

pub type FunctionReturn = Result<Option<AnyValue>, E>;
pub type FunctionExecutor = fn(&mut Function, &mut Context) -> FunctionReturn;

pub trait Implementation {
    fn from(function: &mut Function, cx: &mut Context) -> FunctionReturn;
    fn get_name() -> String;
}

pub fn register(cx: &mut Context) -> Result<(), reader::error::E> {
    cx.add_fn(
        import::Import::get_name(),
        <import::Import as Implementation>::from,
    )?;
    cx.add_fn(os::Os::get_name(), <os::Os as Implementation>::from)?;
    Ok(())
}

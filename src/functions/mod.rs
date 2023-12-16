pub mod reader;
use crate::{context::Context, error::E, reader::entry::Function};

pub trait Implementation<T, O> {
    fn from(function: Function, context: &Context) -> Result<Option<T>, E>;
    fn run(&mut self, context: &mut Context) -> Result<O, E>;
}

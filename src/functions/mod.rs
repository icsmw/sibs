pub mod reader;
use crate::{
    context::Context,
    reader::{entry::Function, error::E},
};

pub trait Implementation<T, O> {
    fn from(function: Function, context: &Context) -> Result<Option<T>, E>;
    fn run(&mut self, context: &mut Context) -> Result<O, E>;
}

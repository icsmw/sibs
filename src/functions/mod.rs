pub mod reader;
use crate::{
    inf::context::Context,
    reader::{entry::Function, error::E},
};

pub trait Implementation<T, O> {
    fn from(function: Function, context: &mut Context) -> Result<Option<T>, E>;
    fn run(&mut self, context: &mut Context) -> Result<O, E>;
}

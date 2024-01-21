pub mod import;
pub mod os;

use crate::{error::E, inf::context::Context, reader::entry::Function};

pub trait Implementation<T, O> {
    fn from(function: Function, context: &mut Context) -> Result<Option<T>, E>;
    fn run(&mut self, context: &mut Context) -> Result<O, E>;
}

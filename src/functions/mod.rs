pub mod reader;
use crate::{error::E, parser::entry::Function};

pub trait Implementation<T, O> {
    fn from(function: Function) -> Result<Option<T>, E>;
    fn run(&mut self) -> Result<O, E>;
}

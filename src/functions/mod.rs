pub mod reader;
use crate::{error::E, reader::entry::Function};

pub trait Implementation<T, O> {
    fn from(function: Function) -> Result<Option<T>, E>;
    fn run(&mut self) -> Result<O, E>;
}

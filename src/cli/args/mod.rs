pub mod help;
pub mod target;

use crate::cli::error::E;
use std::io;

pub trait Argument<T: Sized> {
    fn read(args: &mut Vec<String>) -> Result<Option<T>, E>
    where
        Self: Sized;
    fn post(stdout: &mut io::Stdout) -> Result<(), io::Error>
    where
        Self: Sized;
}

use crate::{cli, reader};
use std::fmt;

#[derive(Debug)]
pub struct LinkedErr<T: fmt::Display> {
    pub token: Option<usize>,
    pub e: T,
}

impl<T: fmt::Display> fmt::Display for LinkedErr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.e)
    }
}

#[derive(Debug)]
pub struct E {
    pub msg: String,
    pub sig: String,
}

impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<cli::error::E> for E {
    fn from(e: cli::error::E) -> Self {
        E {
            msg: e.to_string(),
            sig: String::from("CLI"),
        }
    }
}

impl From<reader::error::E> for E {
    fn from(e: reader::error::E) -> Self {
        E {
            msg: e.to_string(),
            sig: String::from("Reader"),
        }
    }
}

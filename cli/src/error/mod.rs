use crate::{cli, reader};
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub struct LinkedErr<T: fmt::Display> {
    pub token: Option<usize>,
    pub uuid: Uuid,
    pub e: T,
}

#[derive(Debug)]
pub struct LinkedErrSerialized {
    pub e: String,
    pub uuid: Uuid,
    pub token: Option<usize>,
}

impl<T: fmt::Display> LinkedErr<T> {
    pub fn serialize(&self) -> LinkedErrSerialized {
        LinkedErrSerialized {
            e: self.e.to_string(),
            uuid: self.uuid,
            token: self.token.clone(),
        }
    }
}

impl<T: fmt::Display> LinkedErr<T> {
    pub fn new(e: T, token: Option<usize>) -> Self {
        Self {
            token,
            uuid: Uuid::new_v4(),
            e,
        }
    }
    pub fn unlinked(e: T) -> Self {
        Self::new(e, None)
    }
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

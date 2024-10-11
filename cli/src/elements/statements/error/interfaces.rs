use crate::{
    elements::{Error, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", words::ERROR, self.output)
    }
}

impl TokenGetter for Error {
    fn token(&self) -> usize {
        self.token
    }
}

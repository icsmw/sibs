use crate::{
    elements::{TokenGetter, While},
    reader::words,
};
use std::fmt;

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", words::WHILE, self.condition, self.block)
    }
}

impl TokenGetter for While {
    fn token(&self) -> usize {
        self.token
    }
}

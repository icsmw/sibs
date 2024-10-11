use crate::{
    elements::{Loop, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", words::LOOP, self.block)
    }
}

impl TokenGetter for Loop {
    fn token(&self) -> usize {
        self.token
    }
}

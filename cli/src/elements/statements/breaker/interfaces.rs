use crate::{
    elements::{Breaker, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for Breaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", words::BREAK)
    }
}

impl TokenGetter for Breaker {
    fn token(&self) -> usize {
        self.token
    }
}

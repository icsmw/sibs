use crate::elements::{First, TokenGetter};
use std::fmt;

impl fmt::Display for First {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "first {}", self.block)
    }
}

impl TokenGetter for First {
    fn token(&self) -> usize {
        self.token
    }
}

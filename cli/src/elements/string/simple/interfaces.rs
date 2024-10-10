use crate::elements::{SimpleString, TokenGetter};
use std::fmt;

impl fmt::Display for SimpleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl TokenGetter for SimpleString {
    fn token(&self) -> usize {
        self.token
    }
}

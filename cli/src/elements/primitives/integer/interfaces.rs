use crate::elements::{Integer, TokenGetter};
use std::fmt;

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl TokenGetter for Integer {
    fn token(&self) -> usize {
        self.token
    }
}

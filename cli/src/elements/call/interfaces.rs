use crate::elements::{Call, TokenGetter};
use std::fmt;

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{}", self.func)
    }
}

impl TokenGetter for Call {
    fn token(&self) -> usize {
        self.token
    }
}

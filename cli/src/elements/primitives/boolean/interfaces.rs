use crate::elements::{Boolean, TokenGetter};
use std::fmt;

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl TokenGetter for Boolean {
    fn token(&self) -> usize {
        self.token
    }
}

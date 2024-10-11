use crate::elements::{Join, TokenGetter};
use std::fmt;

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "join {}", self.elements)
    }
}

impl TokenGetter for Join {
    fn token(&self) -> usize {
        self.token
    }
}

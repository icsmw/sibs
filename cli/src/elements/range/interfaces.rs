use crate::elements::{Range, TokenGetter};
use std::fmt;

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.from, self.to)
    }
}

impl TokenGetter for Range {
    fn token(&self) -> usize {
        self.token
    }
}

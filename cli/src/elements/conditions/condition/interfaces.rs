use crate::elements::{Condition, TokenGetter};
use std::fmt;

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.subsequence)
    }
}

impl TokenGetter for Condition {
    fn token(&self) -> usize {
        self.token
    }
}

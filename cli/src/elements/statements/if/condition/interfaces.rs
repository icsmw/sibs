use crate::elements::{IfCondition, TokenGetter};
use std::fmt;

impl fmt::Display for IfCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.subsequence)
    }
}

impl TokenGetter for IfCondition {
    fn token(&self) -> usize {
        self.token
    }
}

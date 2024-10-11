use crate::elements::{Each, TokenGetter};
use std::fmt;

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "each({}; {}) {}", self.variable, self.input, self.block)
    }
}

impl TokenGetter for Each {
    fn token(&self) -> usize {
        self.token
    }
}

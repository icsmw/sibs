use crate::elements::{Accessor, TokenGetter};
use std::fmt;

impl fmt::Display for Accessor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.index)
    }
}

impl TokenGetter for Accessor {
    fn token(&self) -> usize {
        self.token
    }
}

use crate::elements::{Optional, TokenGetter};
use std::fmt;

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.condition, self.action)
    }
}

impl TokenGetter for Optional {
    fn token(&self) -> usize {
        self.token
    }
}

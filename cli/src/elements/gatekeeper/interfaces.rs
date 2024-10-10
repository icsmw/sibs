use crate::elements::{Gatekeeper, TokenGetter};
use std::fmt;

impl fmt::Display for Gatekeeper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.function, self.refs)
    }
}

impl TokenGetter for Gatekeeper {
    fn token(&self) -> usize {
        self.token
    }
}

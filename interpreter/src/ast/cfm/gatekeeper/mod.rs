mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Gatekeeper {}

impl fmt::Display for Gatekeeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct OneOf {}

impl fmt::Display for OneOf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

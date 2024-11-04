mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Closure {}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

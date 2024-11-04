mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Call {}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

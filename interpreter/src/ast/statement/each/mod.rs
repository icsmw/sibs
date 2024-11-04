mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Each {}

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

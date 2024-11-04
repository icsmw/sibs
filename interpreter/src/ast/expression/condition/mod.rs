mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Condition {}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

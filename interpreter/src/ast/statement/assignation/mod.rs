mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Assignation {}

impl fmt::Display for Assignation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

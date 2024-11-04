mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Array {}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct For {}

impl fmt::Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct If {}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

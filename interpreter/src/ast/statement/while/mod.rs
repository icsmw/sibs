mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct While {}

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

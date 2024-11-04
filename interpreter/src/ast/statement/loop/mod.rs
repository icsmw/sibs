mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Loop {}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

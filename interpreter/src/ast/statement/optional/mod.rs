mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Optional {}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

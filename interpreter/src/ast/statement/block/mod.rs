mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

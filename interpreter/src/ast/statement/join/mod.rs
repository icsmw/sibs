mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Join {}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

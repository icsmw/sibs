mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Comment {}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

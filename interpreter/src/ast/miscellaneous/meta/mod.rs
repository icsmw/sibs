mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Meta {}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

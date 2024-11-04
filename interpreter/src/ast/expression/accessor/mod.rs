mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Accessor {}

impl fmt::Display for Accessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

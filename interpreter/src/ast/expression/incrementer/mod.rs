mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Incrementer {}

impl fmt::Display for Incrementer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

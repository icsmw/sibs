mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Task {}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

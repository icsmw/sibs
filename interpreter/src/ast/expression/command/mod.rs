mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Command {}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

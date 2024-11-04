mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct InterpolatedString {}

impl fmt::Display for InterpolatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

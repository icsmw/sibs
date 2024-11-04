mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExp {}

impl fmt::Display for BinaryExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

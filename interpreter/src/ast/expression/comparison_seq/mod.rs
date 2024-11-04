mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct ComparisonSeq {}

impl fmt::Display for ComparisonSeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

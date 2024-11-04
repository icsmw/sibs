mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct ComparingSeq {}

impl fmt::Display for ComparingSeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

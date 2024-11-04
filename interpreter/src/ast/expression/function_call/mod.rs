mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionCall {}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

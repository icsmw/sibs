mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableType {}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

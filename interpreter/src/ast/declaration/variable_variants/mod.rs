mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableVariants {}

impl fmt::Display for VariableVariants {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

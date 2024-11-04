mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

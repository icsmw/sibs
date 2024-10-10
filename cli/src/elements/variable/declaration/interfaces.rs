use crate::elements::{TokenGetter, VariableDeclaration};
use std::fmt;

impl TokenGetter for VariableDeclaration {
    fn token(&self) -> usize {
        self.token
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.variable, self.declaration)
    }
}

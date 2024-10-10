use crate::elements::{TokenGetter, VariableName};
use std::fmt;

impl TokenGetter for VariableName {
    fn token(&self) -> usize {
        self.token
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.name)
    }
}

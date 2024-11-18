mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ArgumentDeclaration {
    variable: Box<Node>,
    r#type: Box<Node>,
}

impl fmt::Display for ArgumentDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.variable, self.r#type)
    }
}

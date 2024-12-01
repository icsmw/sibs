#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ArgumentDeclaration {
    pub variable: Box<LinkedNode>,
    pub r#type: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for ArgumentDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.variable, self.r#type)
    }
}

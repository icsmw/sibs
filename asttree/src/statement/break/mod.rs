#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Break {
    pub token: Token,
    pub uuid: Uuid,
}

impl fmt::Display for Break {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<Break> for Node {
    fn from(val: Break) -> Self {
        Node::Statement(Statement::Break(val))
    }
}

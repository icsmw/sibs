#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct IncludeDeclaration {
    pub sig: Token,
    pub from: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for IncludeDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.sig, self.from, self.node)
    }
}

impl From<IncludeDeclaration> for Node {
    fn from(val: IncludeDeclaration) -> Self {
        Node::Declaration(Declaration::IncludeDeclaration(val))
    }
}

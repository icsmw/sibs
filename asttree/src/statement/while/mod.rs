#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct While {
    pub token: Token,
    pub comparison: Box<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for While {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.block)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.comparison)
    }
}

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token, self.comparison, self.block)
    }
}

impl From<While> for Node {
    fn from(val: While) -> Self {
        Node::Statement(Statement::While(val))
    }
}

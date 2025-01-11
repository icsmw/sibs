#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Loop {
    pub token: Token,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for Loop {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.block)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.block)
    }
}

impl From<Loop> for Node {
    fn from(val: Loop) -> Self {
        Node::Statement(Statement::Loop(val))
    }
}

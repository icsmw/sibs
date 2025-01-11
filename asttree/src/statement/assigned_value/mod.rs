#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AssignedValue {
    pub token: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for AssignedValue {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for AssignedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.node)
    }
}

impl From<AssignedValue> for Node {
    fn from(val: AssignedValue) -> Self {
        Node::Statement(Statement::AssignedValue(val))
    }
}

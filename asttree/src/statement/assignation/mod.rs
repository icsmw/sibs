#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Assignation {
    pub left: Box<LinkedNode>,
    pub right: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for Assignation {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.left, &self.right)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Assignation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.left, self.right)
    }
}

impl From<Assignation> for Node {
    fn from(val: Assignation) -> Self {
        Node::Statement(Statement::Assignation(val))
    }
}

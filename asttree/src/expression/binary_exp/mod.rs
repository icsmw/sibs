#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExp {
    pub left: Box<LinkedNode>,
    pub operator: Box<LinkedNode>,
    pub right: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for BinaryExp {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.left, &self.right)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for BinaryExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl From<BinaryExp> for Node {
    fn from(val: BinaryExp) -> Self {
        Node::Expression(Expression::BinaryExp(val))
    }
}

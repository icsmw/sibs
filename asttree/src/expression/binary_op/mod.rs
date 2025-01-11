#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub token: Token,
    pub operator: BinaryOperator,
    pub uuid: Uuid,
}

impl SrcLinking for BinaryOp {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<BinaryOp> for Node {
    fn from(val: BinaryOp) -> Self {
        Node::Expression(Expression::BinaryOp(val))
    }
}

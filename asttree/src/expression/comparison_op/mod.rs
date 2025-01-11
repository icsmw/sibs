#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    BangEqual,
}

#[derive(Debug, Clone)]
pub struct ComparisonOp {
    pub token: Token,
    pub operator: ComparisonOperator,
    pub uuid: Uuid,
}

impl SrcLinking for ComparisonOp {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<ComparisonOp> for Node {
    fn from(val: ComparisonOp) -> Self {
        Node::Expression(Expression::ComparisonOp(val))
    }
}

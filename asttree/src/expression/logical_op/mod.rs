#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, Clone)]
pub struct LogicalOp {
    pub token: Token,
    pub operator: LogicalOperator,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for LogicalOp {
    fn lookup(&'a self, _trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl SrcLinking for LogicalOp {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<LogicalOp> for Node {
    fn from(val: LogicalOp) -> Self {
        Node::Expression(Expression::LogicalOp(val))
    }
}

#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ComparisonSeq {
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for ComparisonSeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<ComparisonSeq> for Node {
    fn from(val: ComparisonSeq) -> Self {
        Node::Expression(Expression::ComparisonSeq(val))
    }
}

#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExpSeq {
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for BinaryExpSeq {
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

impl From<BinaryExpSeq> for Node {
    fn from(val: BinaryExpSeq) -> Self {
        Node::Expression(Expression::BinaryExpSeq(val))
    }
}

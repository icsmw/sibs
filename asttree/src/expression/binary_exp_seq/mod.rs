#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExpSeq {
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for BinaryExpSeq {
    fn link(&self) -> SrcLink {
        if let (Some(open), Some(close)) = (self.nodes.first(), self.nodes.last()) {
            src_from::nodes(open, close)
        } else {
            SrcLink::new(&Uuid::default())
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
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

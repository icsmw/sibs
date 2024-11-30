use crate::*;
use asttree::*;

impl From<&BinaryExpGroup> for SrcLink {
    fn from(node: &BinaryExpGroup) -> Self {
        SrcLink {
            from: node.open.pos.from,
            to: node.close.pos.to,
            src: node.open.src,
        }
    }
}

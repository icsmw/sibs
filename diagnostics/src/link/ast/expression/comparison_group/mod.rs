use crate::*;
use asttree::*;

impl From<&ComparisonGroup> for SrcLink {
    fn from(node: &ComparisonGroup) -> Self {
        SrcLink {
            from: node.open.pos.from,
            to: node.close.pos.to,
            src: node.open.src,
        }
    }
}

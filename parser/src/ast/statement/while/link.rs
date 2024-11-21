use crate::*;
use asttree::*;

impl From<&While> for SrcLink {
    fn from(node: &While) -> Self {
        let block: SrcLink = (&node.block).into();
        SrcLink {
            from: node.token.pos.from,
            to: block.to,
            src: node.token.src,
        }
    }
}

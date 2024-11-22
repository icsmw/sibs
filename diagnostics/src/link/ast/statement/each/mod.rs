use crate::*;
use asttree::*;

impl From<&Each> for SrcLink {
    fn from(node: &Each) -> Self {
        let block: SrcLink = node.block.as_ref().into();
        SrcLink {
            from: node.token.pos.from,
            to: block.to,
            src: node.token.src,
        }
    }
}

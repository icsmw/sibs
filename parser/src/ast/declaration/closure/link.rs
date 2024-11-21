use crate::*;
use asttree::*;

impl From<&Closure> for SrcLink {
    fn from(node: &Closure) -> Self {
        let block: SrcLink = node.block.as_ref().into();
        SrcLink {
            from: node.open.pos.from,
            to: block.to,
            src: block.src,
        }
    }
}

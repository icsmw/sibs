use crate::*;
use asttree::*;

impl From<&Module> for SrcLink {
    fn from(node: &Module) -> Self {
        let n: SrcLink = node.node.as_ref().into();
        SrcLink {
            from: node.token.pos.from,
            to: n.to,
            src: n.src,
        }
    }
}

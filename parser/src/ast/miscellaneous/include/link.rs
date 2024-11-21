use crate::*;
use asttree::*;

impl From<&Include> for SrcLink {
    fn from(node: &Include) -> Self {
        let n: SrcLink = node.node.as_ref().into();
        SrcLink {
            from: node.token.pos.from,
            to: n.to,
            src: n.src,
        }
    }
}

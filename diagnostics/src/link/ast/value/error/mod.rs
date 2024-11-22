use crate::*;
use asttree::*;

impl From<&Error> for SrcLink {
    fn from(node: &Error) -> Self {
        let val: SrcLink = (&node.node).into();
        SrcLink {
            from: node.token.pos.from,
            to: val.to,
            src: node.token.src,
        }
    }
}

use lexer::SrcLink;

use crate::*;

impl From<&Range> for SrcLink {
    fn from(node: &Range) -> Self {
        let left: SrcLink = node.left.as_ref().into();
        let right: SrcLink = node.right.as_ref().into();
        SrcLink {
            from: left.from,
            to: right.to,
            src: right.src,
        }
    }
}

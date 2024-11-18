use lexer::SrcLink;

use crate::*;

impl From<&AssignedValue> for SrcLink {
    fn from(node: &AssignedValue) -> Self {
        let value: SrcLink = node.node.as_ref().into();
        SrcLink {
            from: node.token.pos.from,
            to: value.to,
            src: value.src,
        }
    }
}

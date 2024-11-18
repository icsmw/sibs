use lexer::SrcLink;

use crate::*;

impl From<&Gatekeeper> for SrcLink {
    fn from(node: &Gatekeeper) -> Self {
        let close: SrcLink = (&node.close).into();
        SrcLink {
            from: node.token.pos.from,
            to: close.to,
            src: close.src,
        }
    }
}

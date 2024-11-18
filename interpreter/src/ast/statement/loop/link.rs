use lexer::SrcLink;

use crate::*;

impl From<&Loop> for SrcLink {
    fn from(node: &Loop) -> Self {
        let block: SrcLink = node.block.as_ref().into();
        SrcLink {
            from: node.token.pos.from,
            to: block.to,
            src: node.token.src,
        }
    }
}

use lexer::SrcLink;

use crate::*;

impl From<&For> for SrcLink {
    fn from(node: &For) -> Self {
        let block: SrcLink = node.block.as_ref().into();
        SrcLink {
            from: node.token_for.pos.from,
            to: block.to,
            src: node.token_for.src,
        }
    }
}

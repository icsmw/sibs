use lexer::SrcLink;

use crate::*;

impl From<&Skip> for SrcLink {
    fn from(node: &Skip) -> Self {
        let func: SrcLink = node.func.as_ref().into();
        SrcLink {
            from: node.token.pos.from,
            to: func.to,
            src: func.src,
        }
    }
}

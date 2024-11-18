use lexer::SrcLink;

use crate::*;

impl From<&VariableVariants> for SrcLink {
    fn from(node: &VariableVariants) -> Self {
        if let Some(l) = node.types.last() {
            let l: SrcLink = l.into();
            SrcLink {
                from: node.token.pos.from,
                to: l.to,
                src: l.src,
            }
        } else {
            SrcLink::default()
        }
    }
}

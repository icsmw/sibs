use lexer::SrcLink;

use crate::*;

impl From<&Optional> for SrcLink {
    fn from(node: &Optional) -> Self {
        let open: SrcLink = node.comparison.as_ref().into();
        let close: SrcLink = node.action.as_ref().into();
        SrcLink {
            from: open.from,
            to: close.to,
            src: open.src,
        }
    }
}

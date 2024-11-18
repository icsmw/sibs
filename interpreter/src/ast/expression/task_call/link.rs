use lexer::SrcLink;

use crate::*;

impl From<&TaskCall> for SrcLink {
    fn from(node: &TaskCall) -> Self {
        if let Some((_, tk)) = node.reference.first() {
            (tk, &node.close).into()
        } else {
            SrcLink::default()
        }
    }
}

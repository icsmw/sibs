use lexer::SrcLink;

use crate::*;

impl From<&FunctionCall> for SrcLink {
    fn from(node: &FunctionCall) -> Self {
        if let Some((_, tk)) = node.reference.first() {
            (tk, &node.close).into()
        } else {
            SrcLink::default()
        }
    }
}

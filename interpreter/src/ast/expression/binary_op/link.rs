use lexer::SrcLink;

use crate::*;

impl From<&BinaryOp> for SrcLink {
    fn from(node: &BinaryOp) -> Self {
        (&node.token).into()
    }
}

use lexer::SrcLink;

use crate::*;

impl From<&LogicalOp> for SrcLink {
    fn from(node: &LogicalOp) -> Self {
        (&node.token).into()
    }
}

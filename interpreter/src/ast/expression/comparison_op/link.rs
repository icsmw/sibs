use lexer::SrcLink;

use crate::*;

impl From<&ComparisonOp> for SrcLink {
    fn from(node: &ComparisonOp) -> Self {
        (&node.token).into()
    }
}

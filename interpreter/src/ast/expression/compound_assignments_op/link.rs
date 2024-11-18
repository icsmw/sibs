use lexer::SrcLink;

use crate::*;

impl From<&CompoundAssignmentsOp> for SrcLink {
    fn from(node: &CompoundAssignmentsOp) -> Self {
        (&node.token).into()
    }
}

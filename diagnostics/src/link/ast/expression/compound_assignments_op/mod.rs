use crate::*;
use asttree::*;

impl From<&CompoundAssignmentsOp> for SrcLink {
    fn from(node: &CompoundAssignmentsOp) -> Self {
        (&node.token).into()
    }
}

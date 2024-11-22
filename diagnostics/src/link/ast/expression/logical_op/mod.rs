use crate::*;
use asttree::*;

impl From<&LogicalOp> for SrcLink {
    fn from(node: &LogicalOp) -> Self {
        (&node.token).into()
    }
}

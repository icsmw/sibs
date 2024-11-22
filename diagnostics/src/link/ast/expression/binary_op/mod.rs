use crate::*;
use asttree::*;

impl From<&BinaryOp> for SrcLink {
    fn from(node: &BinaryOp) -> Self {
        (&node.token).into()
    }
}

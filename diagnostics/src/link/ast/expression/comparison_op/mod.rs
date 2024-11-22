use crate::*;
use asttree::*;

impl From<&ComparisonOp> for SrcLink {
    fn from(node: &ComparisonOp) -> Self {
        (&node.token).into()
    }
}

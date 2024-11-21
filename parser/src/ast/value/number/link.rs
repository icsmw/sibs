use crate::*;
use asttree::*;

impl From<&Number> for SrcLink {
    fn from(node: &Number) -> Self {
        (&node.token).into()
    }
}

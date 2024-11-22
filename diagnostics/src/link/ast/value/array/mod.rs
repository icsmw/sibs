use crate::*;
use asttree::*;

impl From<&Array> for SrcLink {
    fn from(node: &Array) -> Self {
        (&node.open, &node.close).into()
    }
}

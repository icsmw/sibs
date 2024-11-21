use crate::*;
use asttree::*;

impl From<&Block> for SrcLink {
    fn from(node: &Block) -> Self {
        (&node.open, &node.close).into()
    }
}

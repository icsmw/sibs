use crate::*;
use asttree::*;

impl From<&Accessor> for SrcLink {
    fn from(node: &Accessor) -> Self {
        (&node.open, &node.close).into()
    }
}

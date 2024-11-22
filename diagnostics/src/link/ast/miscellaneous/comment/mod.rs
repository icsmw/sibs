use crate::*;
use asttree::*;

impl From<&Comment> for SrcLink {
    fn from(node: &Comment) -> Self {
        (&node.token).into()
    }
}

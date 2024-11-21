use crate::*;
use asttree::*;

impl From<&OneOf> for SrcLink {
    fn from(node: &OneOf) -> Self {
        (&node.token, &node.close).into()
    }
}

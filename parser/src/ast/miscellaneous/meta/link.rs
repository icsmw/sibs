use crate::*;
use asttree::*;

impl From<&Meta> for SrcLink {
    fn from(node: &Meta) -> Self {
        (&node.token).into()
    }
}

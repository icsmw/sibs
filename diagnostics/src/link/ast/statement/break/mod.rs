use crate::*;
use asttree::*;

impl From<&Break> for SrcLink {
    fn from(node: &Break) -> Self {
        (&node.token).into()
    }
}

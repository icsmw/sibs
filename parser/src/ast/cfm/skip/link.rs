use crate::*;
use asttree::*;

impl From<&Skip> for SrcLink {
    fn from(node: &Skip) -> Self {
        (&node.token, &node.close).into()
    }
}

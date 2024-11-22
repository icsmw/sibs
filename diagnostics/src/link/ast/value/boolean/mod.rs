use crate::*;
use asttree::*;

impl From<&Boolean> for SrcLink {
    fn from(node: &Boolean) -> Self {
        (&node.token).into()
    }
}

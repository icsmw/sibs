use crate::*;
use asttree::*;

impl From<&Gatekeeper> for SrcLink {
    fn from(node: &Gatekeeper) -> Self {
        (&node.token, &node.close).into()
    }
}

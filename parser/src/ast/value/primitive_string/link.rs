use crate::*;
use asttree::*;

impl From<&PrimitiveString> for SrcLink {
    fn from(node: &PrimitiveString) -> Self {
        (&node.token).into()
    }
}

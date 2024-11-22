use crate::*;
use asttree::*;

impl From<&Variable> for SrcLink {
    fn from(node: &Variable) -> Self {
        (&node.token).into()
    }
}

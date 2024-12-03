use crate::*;
use asttree::*;

impl From<&VariableName> for SrcLink {
    fn from(node: &VariableName) -> Self {
        (&node.token).into()
    }
}

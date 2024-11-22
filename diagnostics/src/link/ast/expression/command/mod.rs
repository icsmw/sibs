use crate::*;
use asttree::*;

impl From<&Command> for SrcLink {
    fn from(node: &Command) -> Self {
        (&node.token).into()
    }
}

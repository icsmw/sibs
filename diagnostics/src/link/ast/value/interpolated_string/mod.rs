use crate::*;
use asttree::*;

impl From<&InterpolatedString> for SrcLink {
    fn from(node: &InterpolatedString) -> Self {
        (&node.token).into()
    }
}

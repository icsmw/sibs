use lexer::SrcLink;

use crate::*;

impl From<&Array> for SrcLink {
    fn from(node: &Array) -> Self {
        (&node.open, &node.close).into()
    }
}

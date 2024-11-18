use lexer::SrcLink;

use crate::*;

impl From<&Accessor> for SrcLink {
    fn from(node: &Accessor) -> Self {
        (&node.open, &node.close).into()
    }
}

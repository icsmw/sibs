use lexer::SrcLink;

use crate::*;

impl From<&Block> for SrcLink {
    fn from(node: &Block) -> Self {
        (&node.open, &node.close).into()
    }
}

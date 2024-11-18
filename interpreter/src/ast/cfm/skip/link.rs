use lexer::SrcLink;

use crate::*;

impl From<&Skip> for SrcLink {
    fn from(node: &Skip) -> Self {
        (&node.token, &node.close).into()
    }
}

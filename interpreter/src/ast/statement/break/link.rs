use lexer::SrcLink;

use crate::*;

impl From<&Break> for SrcLink {
    fn from(node: &Break) -> Self {
        (&node.token).into()
    }
}

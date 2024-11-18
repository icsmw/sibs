use lexer::SrcLink;

use crate::*;

impl From<&Join> for SrcLink {
    fn from(node: &Join) -> Self {
        (&node.token, &node.close).into()
    }
}

use lexer::SrcLink;

use crate::*;

impl From<&Comment> for SrcLink {
    fn from(node: &Comment) -> Self {
        (&node.token).into()
    }
}

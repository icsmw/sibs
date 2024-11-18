use lexer::SrcLink;

use crate::*;

impl From<&OneOf> for SrcLink {
    fn from(node: &OneOf) -> Self {
        (&node.token, &node.close).into()
    }
}

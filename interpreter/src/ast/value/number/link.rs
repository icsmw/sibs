use lexer::SrcLink;

use crate::*;

impl From<&Number> for SrcLink {
    fn from(node: &Number) -> Self {
        (&node.token).into()
    }
}

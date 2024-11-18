use lexer::SrcLink;

use crate::*;

impl From<&Meta> for SrcLink {
    fn from(node: &Meta) -> Self {
        (&node.token).into()
    }
}

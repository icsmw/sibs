use lexer::SrcLink;

use crate::*;

impl From<&PrimitiveString> for SrcLink {
    fn from(node: &PrimitiveString) -> Self {
        (&node.token).into()
    }
}

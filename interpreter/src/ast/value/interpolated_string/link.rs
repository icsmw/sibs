use lexer::SrcLink;

use crate::*;

impl From<&InterpolatedString> for SrcLink {
    fn from(node: &InterpolatedString) -> Self {
        (&node.token).into()
    }
}

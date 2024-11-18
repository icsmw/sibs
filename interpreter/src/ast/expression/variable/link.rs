use lexer::SrcLink;

use crate::*;

impl From<&Variable> for SrcLink {
    fn from(node: &Variable) -> Self {
        (&node.token).into()
    }
}

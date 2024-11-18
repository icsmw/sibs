use lexer::SrcLink;

use crate::*;

impl From<&Gatekeeper> for SrcLink {
    fn from(node: &Gatekeeper) -> Self {
        (&node.token, &node.close).into()
    }
}

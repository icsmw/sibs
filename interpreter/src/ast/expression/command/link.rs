use lexer::SrcLink;

use crate::*;

impl From<&Command> for SrcLink {
    fn from(node: &Command) -> Self {
        (&node.token).into()
    }
}

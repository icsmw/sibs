use lexer::SrcLink;

use crate::*;

impl From<&Root> for SrcLink {
    fn from(node: &Root) -> Self {
        match node {
            Root::Task(n) => n.into(),
            Root::Component(n) => n.into(),
        }
    }
}

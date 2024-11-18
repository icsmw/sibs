use lexer::SrcLink;

use crate::*;

impl From<&Miscellaneous> for SrcLink {
    fn from(node: &Miscellaneous) -> Self {
        match node {
            Miscellaneous::Comment(n) => n.into(),
            Miscellaneous::Include(n) => n.into(),
            Miscellaneous::Meta(n) => n.into(),
            Miscellaneous::Module(n) => n.into(),
        }
    }
}

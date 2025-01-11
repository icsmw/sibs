mod comment;
mod meta;

pub use comment::*;
pub use meta::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Miscellaneous {
    /// /// message
    Meta(Meta),
    /// // comment
    Comment(Comment),
}

impl Miscellaneous {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::Comment(n) => &n.uuid,
            Self::Meta(n) => &n.uuid,
        }
    }
}

impl From<Miscellaneous> for Node {
    fn from(val: Miscellaneous) -> Self {
        Node::Miscellaneous(val)
    }
}

impl SrcLinking for Miscellaneous {
    fn link(&self) -> SrcLink {
        match self {
            Self::Comment(n) => n.link(),
            Self::Meta(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

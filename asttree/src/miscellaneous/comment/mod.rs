#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Comment {
    pub token: Token,
    pub uuid: Uuid,
}

impl SrcLinking for Comment {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<Comment> for Node {
    fn from(val: Comment) -> Self {
        Node::Miscellaneous(Miscellaneous::Comment(val))
    }
}

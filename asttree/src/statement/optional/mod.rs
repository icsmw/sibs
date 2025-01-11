#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Optional {
    pub comparison: Box<LinkedNode>,
    pub token: Token,
    pub action: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for Optional {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.comparison, &self.action)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.comparison, self.token, self.action)
    }
}

impl From<Optional> for Node {
    fn from(val: Optional) -> Self {
        Node::Statement(Statement::Optional(val))
    }
}

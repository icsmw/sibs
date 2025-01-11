#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Accessor {
    pub node: Box<LinkedNode>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl SrcLinking for Accessor {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.open, &self.close)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Accessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.open, self.node, self.close)
    }
}

impl From<Accessor> for Node {
    fn from(val: Accessor) -> Self {
        Node::Expression(Expression::Accessor(val))
    }
}

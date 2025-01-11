#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Range {
    pub left: Box<LinkedNode>,
    pub right: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for Range {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.left, &self.right)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, Kind::DotDot, self.right)
    }
}

impl From<Range> for Node {
    fn from(val: Range) -> Self {
        Node::Expression(Expression::Range(val))
    }
}

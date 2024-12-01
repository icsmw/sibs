#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExpGroup {
    pub open: Token,
    pub close: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for BinaryExpGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.open, self.node, self.close)
    }
}

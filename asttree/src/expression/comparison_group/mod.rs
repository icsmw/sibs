#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ComparisonGroup {
    pub open: Token,
    pub close: Token,
    pub node: Box<Node>,
    pub uuid: Uuid,
}

impl fmt::Display for ComparisonGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.open, self.node, self.close)
    }
}

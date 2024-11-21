#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use lexer::Kind;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Range {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, Kind::DotDot, self.right)
    }
}

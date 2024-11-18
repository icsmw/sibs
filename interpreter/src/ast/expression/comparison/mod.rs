mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use std::fmt;

mod read;

#[derive(Debug, Clone)]
pub struct Comparison {
    pub left: Box<Node>,
    pub operator: Box<Node>,
    pub right: Box<Node>,
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

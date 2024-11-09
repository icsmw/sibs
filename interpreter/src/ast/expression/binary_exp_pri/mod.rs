#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExpPri {
    pub left: Box<Node>,
    pub operator: Box<Node>,
    pub right: Box<Node>,
}

impl fmt::Display for BinaryExpPri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

mod link;
#[cfg(test)]
mod proptests;
mod read;

use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct CompoundAssignments {
    left: Box<Node>,
    operator: Box<Node>,
    right: Box<Node>,
}

impl fmt::Display for CompoundAssignments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

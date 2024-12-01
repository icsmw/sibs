#[cfg(feature = "proptests")]
mod proptests;

use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct CompoundAssignments {
    pub left: Box<LinkedNode>,
    pub operator: Box<LinkedNode>,
    pub right: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for CompoundAssignments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

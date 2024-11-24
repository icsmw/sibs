#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct While {
    pub token: Token,
    pub comparison: Box<Node>,
    pub block: Box<Node>,
    pub uuid: Uuid,
}

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token, self.comparison, self.block)
    }
}

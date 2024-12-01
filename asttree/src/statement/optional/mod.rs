#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Optional {
    pub comparison: Box<LinkedNode>,
    pub token: Token,
    pub action: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.comparison, self.token, self.action)
    }
}

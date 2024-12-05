#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Comment {
    pub token: Token,
    pub uuid: Uuid,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<Comment> for Node {
    fn from(val: Comment) -> Self {
        Node::Miscellaneous(Miscellaneous::Comment(val))
    }
}

#[cfg(feature = "proptests")]
mod proptests;

use lexer::Token;
use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct Optional {
    pub comparison: Box<Node>,
    pub token: Token,
    pub action: Box<Node>,
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.comparison, self.token, self.action)
    }
}

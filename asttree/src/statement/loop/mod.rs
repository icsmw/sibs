#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Loop {
    pub token: Token,
    pub block: Box<Node>,
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.block)
    }
}

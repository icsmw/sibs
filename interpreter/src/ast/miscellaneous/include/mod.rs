mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Include {
    token: Token,
    node: Box<Node>,
}

impl fmt::Display for Include {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.node)
    }
}

mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Accessor {
    node: Box<Node>,
    open: Token,
    close: Token,
}

impl fmt::Display for Accessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.open, self.node, self.close)
    }
}

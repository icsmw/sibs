#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Assignation {
    left: Box<Node>,
    token: Token,
    right: Box<Node>,
}

impl fmt::Display for Assignation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.token, self.right)
    }
}

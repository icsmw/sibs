mod link;
#[cfg(test)]
mod proptests;
mod read;

use lexer::Token;
use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct Optional {
    comparison: Box<Node>,
    token: Token,
    action: Box<Node>,
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.comparison, self.token, self.action)
    }
}

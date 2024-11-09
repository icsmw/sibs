#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Kind;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Accessor {
    node: Box<Node>,
}

impl fmt::Display for Accessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            Kind::LeftBracket,
            self.node,
            Kind::RightBracket
        )
    }
}

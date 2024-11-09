#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    token: Token,
    node: Box<Node>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token,
            Kind::LeftParen,
            self.node,
            Kind::RightParen
        )
    }
}

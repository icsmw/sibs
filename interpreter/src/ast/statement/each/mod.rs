#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Each {
    token: Token,
    element: Box<Node>,
    index: Box<Node>,
    elements: Box<Node>,
    block: Box<Node>,
}

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {}",
            self.token,
            Kind::LeftParen,
            self.element,
            Kind::Comma,
            self.index,
            Kind::Comma,
            self.elements,
            Kind::RightParen,
            self.block
        )
    }
}

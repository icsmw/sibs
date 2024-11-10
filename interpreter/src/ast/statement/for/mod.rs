#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct For {
    token_for: Token,
    token_in: Token,
    element: Box<Node>,
    index: Box<Node>,
    elements: Box<Node>,
    block: Box<Node>,
}

impl fmt::Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {}",
            self.token_for,
            Kind::LeftParen,
            self.element,
            Kind::Comma,
            self.index,
            Kind::RightParen,
            self.token_in,
            self.elements,
            self.block
        )
    }
}

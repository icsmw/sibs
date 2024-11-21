#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Each {
    pub token: Token,
    pub element: Box<Node>,
    pub index: Box<Node>,
    pub elements: Box<Node>,
    pub block: Box<Node>,
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

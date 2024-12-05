#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Each {
    pub token: Token,
    pub element: Box<LinkedNode>,
    pub index: Box<LinkedNode>,
    pub elements: Box<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
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

impl From<Each> for Node {
    fn from(val: Each) -> Self {
        Node::Statement(Statement::Each(val))
    }
}

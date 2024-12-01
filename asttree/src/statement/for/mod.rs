#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct For {
    pub token_for: Token,
    pub token_in: Token,
    pub element: Box<LinkedNode>,
    pub index: Box<LinkedNode>,
    pub elements: Box<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
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

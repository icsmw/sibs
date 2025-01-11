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

impl SrcLinking for For {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token_for, &self.block)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk_and_node(&self.token_for, &self.elements)
    }
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

impl From<For> for Node {
    fn from(val: For) -> Self {
        Node::Statement(Statement::For(val))
    }
}

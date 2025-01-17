#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct For {
    pub token_for: Token,
    pub token_in: Token,
    pub element: Box<LinkedNode>,
    pub index: Option<Box<LinkedNode>>,
    pub elements: Box<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for For {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.element
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.index.as_ref().lookup_inner(self.uuid, trgs))
            .chain(self.elements.lookup_inner(self.uuid, trgs))
            .chain(self.block.lookup_inner(self.uuid, trgs))
            .collect()
    }
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
        if let Some(index) = self.index.as_ref() {
            write!(
                f,
                "{} {} {} {} {} {} {} {} {}",
                self.token_for,
                Kind::LeftParen,
                self.element,
                Kind::Comma,
                index,
                Kind::RightParen,
                self.token_in,
                self.elements,
                self.block
            )
        } else {
            write!(
                f,
                "{} {} {} {} {}",
                self.token_for, self.element, self.token_in, self.elements, self.block
            )
        }
    }
}

impl From<For> for Node {
    fn from(val: For) -> Self {
        Node::Statement(Statement::For(val))
    }
}

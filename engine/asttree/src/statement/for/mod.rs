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

impl Diagnostic for For {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token_for.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.token_for.pos.from, self.block.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        let mut nodes = vec![&*self.element, &*self.elements, &*self.block];
        self.index.as_ref().map(|n| nodes.push(&*n));
        nodes
    }
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

impl FindMutByUuid for For {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.element
            .find_mut_by_uuid(uuid)
            .or_else(|| self.index.find_mut_by_uuid(uuid))
            .or_else(|| self.block.find_mut_by_uuid(uuid))
            .or_else(|| self.elements.find_mut_by_uuid(uuid))
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

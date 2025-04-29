#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct While {
    pub token: Token,
    pub comparison: Box<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for While {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.token.pos.from, self.block.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        vec![&*self.block, &*self.comparison]
    }
}

impl<'a> Lookup<'a> for While {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.comparison
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.block.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for While {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.block
            .find_mut_by_uuid(uuid)
            .or_else(|| self.comparison.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for While {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.block)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.comparison)
    }
}

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token, self.comparison, self.block)
    }
}

impl From<While> for Node {
    fn from(val: While) -> Self {
        Node::Statement(Statement::While(val))
    }
}

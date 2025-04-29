#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Loop {
    pub token: Token,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for Loop {
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
        vec![&*self.block]
    }
}

impl<'a> Lookup<'a> for Loop {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.block.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for Loop {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.block.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for Loop {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.block)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.block)
    }
}

impl From<Loop> for Node {
    fn from(val: Loop) -> Self {
        Node::Statement(Statement::Loop(val))
    }
}

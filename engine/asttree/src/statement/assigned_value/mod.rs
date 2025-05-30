#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AssignedValue {
    pub token: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for AssignedValue {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.token.pos.from, self.node.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        vec![&*self.node]
    }
}

impl<'a> Lookup<'a> for AssignedValue {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for AssignedValue {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for AssignedValue {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for AssignedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.node)
    }
}

impl From<AssignedValue> for Node {
    fn from(val: AssignedValue) -> Self {
        Node::Statement(Statement::AssignedValue(val))
    }
}

#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ArgumentAssignedValue {
    pub token: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for ArgumentAssignedValue {
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

impl<'a> Lookup<'a> for ArgumentAssignedValue {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for ArgumentAssignedValue {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for ArgumentAssignedValue {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for ArgumentAssignedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.node)
    }
}

impl From<ArgumentAssignedValue> for Node {
    fn from(val: ArgumentAssignedValue) -> Self {
        Node::Statement(Statement::ArgumentAssignedValue(val))
    }
}

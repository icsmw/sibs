#[cfg(feature = "proptests")]
mod proptests;

use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct CompoundAssignments {
    pub left: Box<LinkedNode>,
    pub operator: Box<LinkedNode>,
    pub right: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for CompoundAssignments {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.left
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.right.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for CompoundAssignments {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.left
            .find_mut_by_uuid(uuid)
            .or_else(|| self.operator.find_mut_by_uuid(uuid))
            .or_else(|| self.right.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for CompoundAssignments {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.left, &self.right)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for CompoundAssignments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl From<CompoundAssignments> for Node {
    fn from(val: CompoundAssignments) -> Self {
        Node::Expression(Expression::CompoundAssignments(val))
    }
}

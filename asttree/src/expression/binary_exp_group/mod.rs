#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExpGroup {
    pub open: Token,
    pub close: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for BinaryExpGroup {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for BinaryExpGroup {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for BinaryExpGroup {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.open, &self.close)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for BinaryExpGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.open, self.node, self.close)
    }
}

impl From<BinaryExpGroup> for Node {
    fn from(val: BinaryExpGroup) -> Self {
        Node::Expression(Expression::BinaryExpGroup(val))
    }
}

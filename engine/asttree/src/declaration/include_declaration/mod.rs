#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct IncludeDeclaration {
    pub sig: Token,
    pub from: Token,
    pub node: Box<LinkedNode>,
    pub root: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for IncludeDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for IncludeDeclaration {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for IncludeDeclaration {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.sig, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for IncludeDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.sig, self.from, self.node)
    }
}

impl From<IncludeDeclaration> for Node {
    fn from(val: IncludeDeclaration) -> Self {
        Node::Declaration(Declaration::IncludeDeclaration(val))
    }
}

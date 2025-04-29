#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ModuleDeclaration {
    pub sig: Token,
    pub from: Token,
    pub node: Box<LinkedNode>,
    pub name: String,
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for ModuleDeclaration {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.sig.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.sig.pos.from, self.node.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        let mut nodes: Vec<&LinkedNode> = self.nodes.iter().collect();
        nodes.push(&*self.node);
        nodes
    }
}

impl<'a> Lookup<'a> for ModuleDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for ModuleDeclaration {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for ModuleDeclaration {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.sig, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for ModuleDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.sig, self.from, self.node)
    }
}

impl From<ModuleDeclaration> for Node {
    fn from(val: ModuleDeclaration) -> Self {
        Node::Declaration(Declaration::ModuleDeclaration(val))
    }
}

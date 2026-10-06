mod body;
#[cfg(feature = "proptests")]
mod proptests;

pub use body::*;

use crate::*;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct ModuleImport {
    pub sig: Token,
    pub from: Token,
    pub node: Box<LinkedNode>,
    pub name: String,
    pub body: Arc<ModuleBody>,
    pub uuid: Uuid,
}

impl Diagnostic for ModuleImport {
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
        let mut nodes: Vec<&LinkedNode> = self.body.nodes.iter().collect();
        nodes.push(&*self.node);
        nodes
    }
}

impl<'a> Lookup<'a> for ModuleImport {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for ModuleImport {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for ModuleImport {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.sig, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for ModuleImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.sig, self.from, self.node)
    }
}

impl From<ModuleImport> for Node {
    fn from(val: ModuleImport) -> Self {
        Node::Declaration(Declaration::ModuleImport(val))
    }
}

impl Extract for ModuleImport {
    fn extract(node: &Node) -> Option<&Self> {
        match Declaration::extract(node)? {
            Declaration::ModuleImport(node) => Some(node),
            _ => None,
        }
    }
}

impl MetadataContent for ModuleImport {
    fn md_includes() -> &'static [MiscellaneousId] {
        &[MiscellaneousId::Meta, MiscellaneousId::Comment]
    }
}

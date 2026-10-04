#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EnvsImport {
    pub sig: Token,
    pub from: Token,
    pub node: Box<LinkedNode>,
    pub root: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for EnvsImport {
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
        vec![&*self.node, &*self.root]
    }
}

impl<'a> Lookup<'a> for EnvsImport {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.root.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for EnvsImport {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node
            .find_mut_by_uuid(uuid)
            .or_else(|| self.root.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for EnvsImport {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.sig, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for EnvsImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.sig, self.from, self.node)
    }
}

impl From<EnvsImport> for Node {
    fn from(val: EnvsImport) -> Self {
        Node::Root(Root::EnvsImport(val))
    }
}

impl Extract for EnvsImport {
    fn extract(node: &Node) -> Option<&Self> {
        match Root::extract(node)? {
            Root::EnvsImport(node) => Some(node),
            _ => None,
        }
    }
}

impl MetadataContent for EnvsImport {
    fn md_includes() -> &'static [MiscellaneousId] {
        &[MiscellaneousId::Meta, MiscellaneousId::Comment]
    }
}

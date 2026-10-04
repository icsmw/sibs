#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GlobalsModule {
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for GlobalsModule {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        let Some(first) = self.nodes.first() else {
            return false;
        };
        if !first.md.link.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        if let (Some(first), Some(last)) = (self.nodes.first(), self.nodes.last()) {
            Position::new(first.md.link.from(), last.md.link.to())
        } else {
            Position::new(TextPosition::default(), TextPosition::default())
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        self.nodes.iter().collect()
    }
}

impl<'a> Lookup<'a> for GlobalsModule {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.nodes
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for GlobalsModule {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.nodes.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for GlobalsModule {
    fn link(&self) -> SrcLink {
        if let (Some(open), Some(close)) = (self.nodes.first(), self.nodes.last()) {
            src_from::nodes(open, close)
        } else {
            SrcLink::new(&self.uuid)
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for GlobalsModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.nodes {
            if matches!(node.get_node(), Node::Miscellaneous(..)) {
                write!(f, "\n{node}\n")?;
            } else {
                write!(f, "{node} ; ")?;
            }
        }
        Ok(())
    }
}

impl From<GlobalsModule> for Node {
    fn from(val: GlobalsModule) -> Self {
        Node::Root(Root::GlobalsModule(val))
    }
}

impl Extract for GlobalsModule {
    fn extract(node: &Node) -> Option<&Self> {
        match Root::extract(node)? {
            Root::GlobalsModule(node) => Some(node),
            _ => None,
        }
    }
}

impl MetadataContent for GlobalsModule {
    fn md_includes() -> &'static [MiscellaneousId] {
        &[]
    }
}

#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Anchor {
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl Anchor {
    pub fn get_component<S: AsRef<str>>(&self, name: S) -> Option<&LinkedNode> {
        self.nodes.iter().find(|n| {
            if let Node::Root(Root::Component(comp)) = &n.node {
                name.as_ref() == comp.get_name()
            } else {
                false
            }
        })
    }
}

impl<'a> Lookup<'a> for Anchor {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.nodes
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for Anchor {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.nodes.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for Anchor {
    fn link(&self) -> SrcLink {
        if let (Some(open), Some(close)) = (self.nodes.first(), self.nodes.last()) {
            src_from::nodes(open, close)
        } else {
            SrcLink::new(&Uuid::default())
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Semicolon)),
        )
    }
}

impl From<Anchor> for Node {
    fn from(val: Anchor) -> Self {
        Node::Root(Root::Anchor(val))
    }
}

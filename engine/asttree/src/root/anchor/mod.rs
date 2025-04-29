#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Anchor {
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl Anchor {
    pub fn get_component<S: AsRef<str>>(&self, name: S) -> Option<&LinkedNode> {
        self.nodes.iter().find_map(|n| match &n.node {
            Node::Root(Root::Component(comp)) => {
                if name.as_ref() == comp.get_name() {
                    Some(n)
                } else {
                    None
                }
            }
            Node::Declaration(Declaration::IncludeDeclaration(incl)) => {
                incl.get_component(name.as_ref())
            }
            _ => None,
        })
    }
    pub fn get_components_md(&self) -> AnchorMetadata {
        let mut components = HashMap::new();
        self.nodes.iter().for_each(|n| match &n.node {
            Node::Root(Root::Component(comp)) => {
                components.insert(comp.get_name(), (&n.md, comp.get_tasks_md()));
            }
            Node::Declaration(Declaration::IncludeDeclaration(incl)) => {
                components.extend(incl.get_components_md());
            }
            _ => {}
        });
        components
    }
}

impl Diagnostic for Anchor {
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
            Position::new(0, 0)
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        self.nodes.iter().collect()
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

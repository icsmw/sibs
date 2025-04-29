#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Token,
    pub sig: Token,
    pub open: Token,
    pub close: Token,
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl Module {
    pub fn get_name(&self) -> Option<&str> {
        if let Kind::Identifier(name) = &self.name.kind {
            Some(name)
        } else {
            None
        }
    }
}

impl Diagnostic for Module {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.sig.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::tokens(&self.sig, &self.close)
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        self.nodes.iter().collect()
    }
}

impl<'a> Lookup<'a> for Module {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.nodes
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for Module {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.nodes.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for Module {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.sig, &self.close)
    }
    fn slink(&self) -> SrcLink {
        src_from::tks(&self.sig, &self.name)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.sig,
            self.name,
            self.open,
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Semicolon)),
            self.close
        )
    }
}

impl From<Module> for Node {
    fn from(val: Module) -> Self {
        Node::Root(Root::Module(val))
    }
}

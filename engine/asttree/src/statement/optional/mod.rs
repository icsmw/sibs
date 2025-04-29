#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Optional {
    pub comparison: Box<LinkedNode>,
    pub token: Token,
    pub action: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for Optional {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.comparison.md.link.from(), self.action.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        vec![&*self.comparison, &*self.action]
    }
}

impl<'a> Lookup<'a> for Optional {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.comparison
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.action.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for Optional {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.comparison
            .find_mut_by_uuid(uuid)
            .or_else(|| self.action.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for Optional {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.comparison, &self.action)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.comparison, self.token, self.action)
    }
}

impl From<Optional> for Node {
    fn from(val: Optional) -> Self {
        Node::Statement(Statement::Optional(val))
    }
}

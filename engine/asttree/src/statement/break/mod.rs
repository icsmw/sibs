#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Break {
    pub token: Token,
    pub target: Option<Uuid>,
    pub uuid: Uuid,
}

impl Break {
    pub fn set_target(&mut self, uuid: &Uuid) {
        if self.target.is_none() {
            self.target = Some(*uuid)
        }
    }
    pub fn is_target(&self, uuid: &Uuid) -> bool {
        self.target
            .as_ref()
            .map(|target| target == uuid)
            .unwrap_or(false)
    }
    pub fn is_assigned(&self) -> bool {
        self.target.is_some()
    }
}

impl<'a> Lookup<'a> for Break {
    fn lookup(&'a self, _trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl FindMutByUuid for Break {
    fn find_mut_by_uuid(&mut self, _uuid: &Uuid) -> Option<&mut LinkedNode> {
        None
    }
}

impl SrcLinking for Break {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Break {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<Break> for Node {
    fn from(val: Break) -> Self {
        Node::Statement(Statement::Break(val))
    }
}

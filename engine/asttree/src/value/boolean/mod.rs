#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub inner: bool,
    pub token: Token,
    pub uuid: Uuid,
}

impl Diagnostic for Boolean {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        self.token.pos.clone()
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        Vec::new()
    }
}

impl<'a> Lookup<'a> for Boolean {
    fn lookup(&'a self, _trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl FindMutByUuid for Boolean {
    fn find_mut_by_uuid(&mut self, _uuid: &Uuid) -> Option<&mut LinkedNode> {
        None
    }
}

impl SrcLinking for Boolean {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<Boolean> for Node {
    fn from(val: Boolean) -> Self {
        Node::Value(Value::Boolean(val))
    }
}

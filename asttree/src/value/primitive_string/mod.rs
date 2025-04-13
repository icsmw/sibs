#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PrimitiveString {
    pub inner: String,
    pub token: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for PrimitiveString {
    fn lookup(&'a self, _trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl FindMutByUuid for PrimitiveString {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        None
    }
}

impl SrcLinking for PrimitiveString {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for PrimitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<PrimitiveString> for Node {
    fn from(val: PrimitiveString) -> Self {
        Node::Value(Value::PrimitiveString(val))
    }
}

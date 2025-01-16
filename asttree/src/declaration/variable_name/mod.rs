#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableName {
    pub ident: String,
    pub token: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for VariableName {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl SrcLinking for VariableName {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<VariableName> for Node {
    fn from(val: VariableName) -> Self {
        Node::Declaration(Declaration::VariableName(val))
    }
}

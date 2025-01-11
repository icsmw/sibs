#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ModuleDeclaration {
    pub sig: Token,
    pub from: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl SrcLinking for ModuleDeclaration {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.sig, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for ModuleDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.sig, self.from, self.node)
    }
}

impl From<ModuleDeclaration> for Node {
    fn from(val: ModuleDeclaration) -> Self {
        Node::Declaration(Declaration::ModuleDeclaration(val))
    }
}

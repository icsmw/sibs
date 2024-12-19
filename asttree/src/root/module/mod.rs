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

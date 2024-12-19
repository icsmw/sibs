#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Include {
    pub src: Uuid,
    pub sig: Token,
    pub open: Token,
    pub close: Token,
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
}

impl fmt::Display for Include {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.sig,
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

impl From<Include> for Node {
    fn from(val: Include) -> Self {
        Node::Root(Root::Include(val))
    }
}

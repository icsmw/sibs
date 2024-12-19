#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Anchor {
    pub nodes: Vec<LinkedNode>,
    pub uuid: Uuid,
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

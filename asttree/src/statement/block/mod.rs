#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    pub nodes: Vec<LinkedNode>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl SrcLinking for Block {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.open, &self.close)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.open,
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Semicolon)),
            if self.nodes.is_empty() {
                String::new()
            } else {
                Kind::Semicolon.to_string()
            },
            self.close
        )
    }
}

impl From<Block> for Node {
    fn from(val: Block) -> Self {
        Node::Statement(Statement::Block(val))
    }
}

mod read;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    pub nodes: Vec<Node>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

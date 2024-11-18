mod link;
#[cfg(test)]
mod proptests;
mod read;

use lexer::Kind;

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
            "{} {} {} {}",
            Kind::LeftBrace,
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
            Kind::RightBrace
        )
    }
}

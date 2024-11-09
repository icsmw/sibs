#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Kind;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BinaryExpGroup {
    pub nodes: Vec<Node>,
}

impl fmt::Display for BinaryExpGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            Kind::LeftParen,
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            Kind::RightParen
        )
    }
}

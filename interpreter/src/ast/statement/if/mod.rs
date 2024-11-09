#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{KindId, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub enum IfCase {
    /// (Node::Expression::ComparisonSeq, Node::Statement::Block, Token)
    If(Node, Node, Token),
    /// (Node::Statement::Block, Token)
    Else(Node, Token),
}

impl fmt::Display for IfCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If(condition, block, _) => {
                    format!("{} {condition} {block} ", KindId::If,)
                }
                Self::Else(block, _) => {
                    format!("{} {block} ", KindId::Else,)
                }
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub cases: Vec<IfCase>,
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cases
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

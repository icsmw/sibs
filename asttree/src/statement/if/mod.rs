#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum IfCase {
    /// (LinkedNode::Expression::ComparisonSeq, LinkedNode::Statement::Block, Token)
    If(LinkedNode, LinkedNode, Token),
    /// (LinkedNode::Statement::Block, Token)
    Else(LinkedNode, Token),
}

impl fmt::Display for IfCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If(condition, block, _) => {
                    format!("{} {condition} {block} ", Keyword::If,)
                }
                Self::Else(block, _) => {
                    format!("{} {block} ", Keyword::Else,)
                }
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub cases: Vec<IfCase>,
    pub uuid: Uuid,
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

impl From<If> for Node {
    fn from(val: If) -> Self {
        Node::Statement(Statement::If(val))
    }
}

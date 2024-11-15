#[cfg(test)]
mod proptests;
mod read;

use lexer::{Kind, Token};
use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct Component {
    sig: Token,
    name: Token,
    path: String,
    nodes: Vec<Node>,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            self.sig,
            self.name,
            Kind::LeftParen,
            self.path,
            Kind::RightParen,
            Kind::LeftBrace,
            self.nodes
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            Kind::RightBrace
        )
    }
}

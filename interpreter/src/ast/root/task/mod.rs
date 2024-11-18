mod link;
#[cfg(test)]
mod proptests;
mod read;

use lexer::{Kind, Token};
use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct Task {
    vis: Option<Token>,
    sig: Token,
    name: Token,
    args: Vec<Node>,
    block: Box<Node>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {} {} {} {} {}",
            self.vis
                .as_ref()
                .map(|vis| format!("{vis} "))
                .unwrap_or_default(),
            self.sig,
            self.name,
            Kind::LeftParen,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightParen,
            self.block
        )
    }
}

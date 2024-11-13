#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Kind;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Closure {
    args: Vec<Node>,
    block: Box<Node>,
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
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

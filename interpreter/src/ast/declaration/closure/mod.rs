mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Closure {
    args: Vec<Node>,
    block: Box<Node>,
    open: Token,
    close: Token,
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.open,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            self.close,
            self.block
        )
    }
}

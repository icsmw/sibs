mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Gatekeeper {
    token: Token,
    nodes: Vec<Node>,
    open: Token,
    close: Token,
}

impl fmt::Display for Gatekeeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token,
            self.open,
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            self.close
        )
    }
}

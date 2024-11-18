mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Return {
    token: Token,
    node: Option<Box<Node>>,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.token,
            self.node
                .as_ref()
                .map(|n| format!(" {n}"))
                .unwrap_or_default()
        )
    }
}

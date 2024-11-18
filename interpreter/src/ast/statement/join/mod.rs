mod link;
#[cfg(test)]
mod proptests;
mod read;

use lexer::{Kind, Token};
use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct Join {
    commands: Vec<Node>,
    token: Token,
    open: Token,
    close: Token,
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token,
            self.open,
            self.commands
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(&Kind::Comma.to_string()),
            self.close
        )
    }
}

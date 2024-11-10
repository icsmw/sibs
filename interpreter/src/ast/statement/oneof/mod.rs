#[cfg(test)]
mod proptests;
mod read;

use lexer::{Kind, Token};
use std::fmt;

use crate::*;

#[derive(Debug, Clone)]
pub struct OneOf {
    commands: Vec<Node>,
    token: Token,
}

impl fmt::Display for OneOf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token,
            Kind::LeftParen,
            self.commands
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(&Kind::Comma.to_string()),
            Kind::RightParen
        )
    }
}

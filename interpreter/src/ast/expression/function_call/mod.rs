#[cfg(test)]
mod proptests;
mod read;

use lexer::{Kind, Token};

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub args: Vec<Node>,
    pub reference: Vec<(String, Token)>,
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.reference
                .iter()
                .map(|(s, _)| s.to_owned())
                .collect::<Vec<String>>()
                .join(&Kind::Colon.to_string().repeat(2)),
            Kind::LeftParen,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightParen
        )
    }
}

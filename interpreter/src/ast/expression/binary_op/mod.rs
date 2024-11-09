#[cfg(test)]
mod proptests;
mod read;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub token: Token,
    pub operator: BinaryOperator,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

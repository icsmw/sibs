use crate::*;
use lexer::Token;
use std::fmt;

mod read;

#[derive(Debug, Clone)]
enum Side {
    Number(i64, Token),
    Variable(String, Token),
    Bool(bool, Token),
}

#[derive(Debug, Clone)]
pub struct Comparing {
    pub left: Side,
    pub operator: Box<Node>,
    pub right: Side,
}

impl fmt::Display for Comparing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

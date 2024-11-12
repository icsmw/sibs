mod read;

use crate::*;
use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    token: Token,
    variable: Token,
    r#type: Option<Box<Node>>,
    value: Option<Box<Node>>,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    token: Token,
    variable: Box<Node>,
    r#type: Option<Box<Node>>,
    assignation: Option<Box<Node>>,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}{}{}",
            self.token,
            self.variable,
            self.r#type
                .as_ref()
                .map(|ty| format!(" {ty}"))
                .unwrap_or_default(),
            self.assignation
                .as_ref()
                .map(|ty| format!(" {ty}"))
                .unwrap_or_default()
        )
    }
}

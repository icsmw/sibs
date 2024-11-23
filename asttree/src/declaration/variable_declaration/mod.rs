#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub token: Token,
    pub variable: Box<Node>,
    pub r#type: Option<Box<Node>>,
    pub assignation: Option<Box<Node>>,
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

#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub token: Token,
    pub variable: Box<LinkedNode>,
    pub r#type: Option<Box<LinkedNode>>,
    pub assignation: Option<Box<LinkedNode>>,
    pub uuid: Uuid,
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

impl From<VariableDeclaration> for Node {
    fn from(val: VariableDeclaration) -> Self {
        Node::Declaration(Declaration::VariableDeclaration(val))
    }
}

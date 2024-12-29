#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ArgumentDeclaration {
    pub variable: Box<LinkedNode>,
    pub r#type: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl ArgumentDeclaration {
    pub fn get_var_name(&self) -> Option<String> {
        if let Node::Declaration(Declaration::VariableName(n)) = &self.variable.node {
            Some(n.ident.clone())
        } else {
            None
        }
    }
}
impl fmt::Display for ArgumentDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.variable, self.r#type)
    }
}

impl From<ArgumentDeclaration> for Node {
    fn from(val: ArgumentDeclaration) -> Self {
        Node::Declaration(Declaration::ArgumentDeclaration(val))
    }
}

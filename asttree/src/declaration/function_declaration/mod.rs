#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub sig: Token,
    pub name: Token,
    pub args: Vec<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl FunctionDeclaration {
    pub fn get_name(&self) -> Option<&str> {
        if let Kind::Identifier(name) = &self.name.kind {
            Some(name)
        } else {
            None
        }
    }
}

impl fmt::Display for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.sig,
            self.name,
            Kind::LeftParen,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightParen,
            self.block
        )
    }
}

impl From<FunctionDeclaration> for Node {
    fn from(val: FunctionDeclaration) -> Self {
        Node::Declaration(Declaration::FunctionDeclaration(val))
    }
}

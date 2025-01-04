#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ClosureDeclaration {
    pub args: Vec<LinkedNode>,
    pub ty: Box<LinkedNode>,
    pub token: Token,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl fmt::Display for ClosureDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.token,
            self.open,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            self.close,
            self.ty
        )
    }
}

impl From<ClosureDeclaration> for Node {
    fn from(val: ClosureDeclaration) -> Self {
        Node::Declaration(Declaration::ClosureDeclaration(val))
    }
}

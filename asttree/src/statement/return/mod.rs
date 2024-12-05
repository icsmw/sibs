#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Return {
    pub token: Token,
    pub node: Option<Box<LinkedNode>>,
    pub uuid: Uuid,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.token,
            self.node
                .as_ref()
                .map(|n| format!(" {n}"))
                .unwrap_or_default()
        )
    }
}

impl From<Return> for Node {
    fn from(val: Return) -> Self {
        Node::Statement(Statement::Return(val))
    }
}

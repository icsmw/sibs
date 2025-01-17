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

impl<'a> Lookup<'a> for Return {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.as_ref().lookup_inner(self.uuid, trgs)
    }
}

impl SrcLinking for Return {
    fn link(&self) -> SrcLink {
        if let Some(n) = self.node.as_ref() {
            src_from::tk_and_node(&self.token, n)
        } else {
            src_from::tk(&self.token)
        }
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
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

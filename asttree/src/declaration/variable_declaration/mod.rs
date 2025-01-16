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

impl<'a> Lookup<'a> for VariableDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.variable
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.r#type.as_ref().lookup_inner(self.uuid, trgs))
            .chain(self.assignation.as_ref().lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl SrcLinking for VariableDeclaration {
    fn link(&self) -> SrcLink {
        if let Some(node) = self.assignation.as_ref() {
            src_from::tk_and_node(&self.token, node)
        } else if let Some(node) = self.r#type.as_ref() {
            src_from::tk_and_node(&self.token, node)
        } else {
            src_from::tk_and_node(&self.token, &self.variable)
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
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

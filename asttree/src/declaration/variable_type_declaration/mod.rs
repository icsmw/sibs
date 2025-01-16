#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableTypeDeclaration {
    pub types: Vec<LinkedNode>,
    pub token: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for VariableTypeDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.types
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl SrcLinking for VariableTypeDeclaration {
    fn link(&self) -> SrcLink {
        if let Some(n) = self.types.last() {
            src_from::tk_and_node(&self.token, n)
        } else {
            src_from::tk(&self.token)
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for VariableTypeDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.token,
            self.types
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::VerticalBar))
        )
    }
}

impl From<VariableTypeDeclaration> for Node {
    fn from(val: VariableTypeDeclaration) -> Self {
        Node::Declaration(Declaration::VariableTypeDeclaration(val))
    }
}

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

impl<'a> Lookup<'a> for ArgumentDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.variable
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.r#type.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for ArgumentDeclaration {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.variable
            .find_mut_by_uuid(uuid)
            .or_else(|| self.r#type.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for ArgumentDeclaration {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.variable, &self.r#type)
    }
    fn slink(&self) -> SrcLink {
        self.link()
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

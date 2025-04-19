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

impl<'a> Lookup<'a> for ClosureDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.args
            .iter()
            .flat_map(|arg| arg.lookup_inner(self.uuid, trgs))
            .collect::<Vec<FoundNode>>()
            .into_iter()
            .chain(self.ty.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for ClosureDeclaration {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.ty
            .find_mut_by_uuid(uuid)
            .or_else(|| self.args.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for ClosureDeclaration {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.open, &self.ty)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
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

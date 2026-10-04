#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GlobalDeclaration {
    pub token: Token,
    pub variable: Box<LinkedNode>,
    pub r#type: Box<LinkedNode>,
    pub assignation: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl GlobalDeclaration {
    pub fn is_mutable(&self) -> bool {
        matches!(self.token.kind, Kind::Keyword(Keyword::Global))
    }
}

impl Diagnostic for GlobalDeclaration {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.token.pos.from, self.assignation.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        vec![&self.variable, &self.r#type, &self.assignation]
    }
}

impl<'a> Lookup<'a> for GlobalDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.variable
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.r#type.lookup_inner(self.uuid, trgs))
            .chain(self.assignation.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for GlobalDeclaration {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.variable
            .find_mut_by_uuid(uuid)
            .or_else(|| self.r#type.find_mut_by_uuid(uuid))
            .or_else(|| self.assignation.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for GlobalDeclaration {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.assignation)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for GlobalDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token, self.variable, self.r#type, self.assignation
        )
    }
}

impl From<GlobalDeclaration> for Node {
    fn from(val: GlobalDeclaration) -> Self {
        Node::Declaration(Declaration::GlobalDeclaration(val))
    }
}

impl Extract for GlobalDeclaration {
    fn extract(node: &Node) -> Option<&Self> {
        match Declaration::extract(node)? {
            Declaration::GlobalDeclaration(node) => Some(node),
            _ => None,
        }
    }
}

impl MetadataContent for GlobalDeclaration {
    fn md_includes() -> &'static [MiscellaneousId] {
        &[MiscellaneousId::Meta, MiscellaneousId::Comment]
    }
}

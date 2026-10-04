#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EnvDeclaration {
    pub token: Token,
    pub variable: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl EnvDeclaration {
    pub fn is_optional(&self) -> bool {
        matches!(self.token.kind, Kind::Keyword(Keyword::Optional))
    }
}

impl Diagnostic for EnvDeclaration {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.token.pos.from, self.variable.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        vec![&self.variable]
    }
}

impl<'a> Lookup<'a> for EnvDeclaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.variable.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for EnvDeclaration {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.variable.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for EnvDeclaration {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.variable)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for EnvDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token, self.variable)
    }
}

impl From<EnvDeclaration> for Node {
    fn from(val: EnvDeclaration) -> Self {
        Node::Declaration(Declaration::EnvDeclaration(val))
    }
}

impl Extract for EnvDeclaration {
    fn extract(node: &Node) -> Option<&Self> {
        match Declaration::extract(node)? {
            Declaration::EnvDeclaration(node) => Some(node),
            _ => None,
        }
    }
}

impl MetadataContent for EnvDeclaration {
    fn md_includes() -> &'static [MiscellaneousId] {
        &[]
    }
}

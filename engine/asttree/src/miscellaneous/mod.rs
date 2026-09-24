mod comment;
mod meta;
mod root_meta;

pub use comment::*;
pub use meta::*;
pub use root_meta::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Miscellaneous {
    /// /// message
    Meta(Meta),
    /// //! message
    RootMeta(RootMeta),
    /// // comment
    Comment(Comment),
}

impl Identification for Miscellaneous {
    fn uuid(&self) -> &Uuid {
        match self {
            Self::Comment(n) => &n.uuid,
            Self::Meta(n) => &n.uuid,
            Self::RootMeta(n) => &n.uuid,
        }
    }
    fn ident(&self) -> String {
        match self {
            Self::Comment(..) => MiscellaneousId::Comment.to_string(),
            Self::Meta(..) => MiscellaneousId::Meta.to_string(),
            Self::RootMeta(..) => MiscellaneousId::RootMeta.to_string(),
        }
    }
}

impl Diagnostic for Miscellaneous {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        match self {
            Self::Comment(n) => n.located(src, pos),
            Self::Meta(n) => n.located(src, pos),
            Self::RootMeta(n) => n.located(src, pos),
        }
    }
    fn get_position(&self) -> Position {
        match self {
            Self::Comment(n) => n.get_position(),
            Self::Meta(n) => n.get_position(),
            Self::RootMeta(n) => n.get_position(),
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        match self {
            Self::Comment(n) => n.childs(),
            Self::Meta(n) => n.childs(),
            Self::RootMeta(n) => n.childs(),
        }
    }
}

impl From<Miscellaneous> for Node {
    fn from(val: Miscellaneous) -> Self {
        Node::Miscellaneous(val)
    }
}

impl<'a> Lookup<'a> for Miscellaneous {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            Self::Comment(n) => n.lookup(trgs),
            Self::Meta(n) => n.lookup(trgs),
            Self::RootMeta(n) => n.lookup(trgs),
        }
    }
}

impl FindMutByUuid for Miscellaneous {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            Self::Comment(n) => n.find_mut_by_uuid(uuid),
            Self::Meta(n) => n.find_mut_by_uuid(uuid),
            Self::RootMeta(n) => n.find_mut_by_uuid(uuid),
        }
    }
}

impl SrcLinking for Miscellaneous {
    fn link(&self) -> SrcLink {
        match self {
            Self::Comment(n) => n.link(),
            Self::Meta(n) => n.link(),
            Self::RootMeta(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl Extract for Miscellaneous {
    fn extract(node: &Node) -> Option<&Self> {
        match node {
            Node::Miscellaneous(node) => Some(node),
            _ => None,
        }
    }
}

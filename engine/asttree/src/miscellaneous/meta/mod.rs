#[cfg(feature = "proptests")]
mod proptests;
use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Meta {
    pub token: Token,
    pub uuid: Uuid,
}

impl Meta {
    pub fn as_trimmed_string(&self) -> String {
        self.token.to_string().replace("///", "").trim().to_owned()
    }
}

impl<'a> Lookup<'a> for Meta {
    fn lookup(&'a self, _trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl FindMutByUuid for Meta {
    fn find_mut_by_uuid(&mut self, _uuid: &Uuid) -> Option<&mut LinkedNode> {
        None
    }
}

impl SrcLinking for Meta {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", Kind::LF, self.token, Kind::LF)
    }
}

impl From<Meta> for Node {
    fn from(val: Meta) -> Self {
        Node::Miscellaneous(Miscellaneous::Meta(val))
    }
}

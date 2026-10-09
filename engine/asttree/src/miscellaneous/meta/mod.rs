#[cfg(feature = "proptests")]
mod proptests;
use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Meta {
    pub token: Token,
    pub uuid: Uuid,
}

impl Diagnostic for Meta {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        self.token.pos.clone()
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        Vec::new()
    }
}

impl Meta {
    /// Documentation text without the marker and one optional formatting space.
    /// Markdown indentation and trailing spaces remain significant.
    pub fn as_doc_str(&self) -> &str {
        let Kind::Meta(content) = &self.token.kind else {
            return "";
        };
        let content = content.strip_suffix('\r').unwrap_or(content);
        content.strip_prefix(' ').unwrap_or(content)
    }

    pub fn as_trimmed_string(&self) -> String {
        self.as_doc_str().trim().to_owned()
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

impl Extract for Meta {
    fn extract(node: &Node) -> Option<&Self> {
        match Miscellaneous::extract(node)? {
            Miscellaneous::Meta(node) => Some(node),
            _ => None,
        }
    }
}

impl MetadataContent for Meta {
    fn md_includes() -> &'static [MiscellaneousId] {
        &[]
    }
}

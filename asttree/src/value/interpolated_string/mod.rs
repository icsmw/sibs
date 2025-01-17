#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum InterpolatedStringPart {
    Open(Token),
    Literal(String),
    Expression(LinkedNode),
    Close(Token),
}

impl<'a> LookupInner<'a> for &'a InterpolatedStringPart {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            InterpolatedStringPart::Open(..)
            | InterpolatedStringPart::Close(..)
            | InterpolatedStringPart::Literal(..) => Vec::new(),
            InterpolatedStringPart::Expression(n) => n.lookup_inner(owner, trgs),
        }
    }
}

impl fmt::Display for InterpolatedStringPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Open(tk) => tk.to_string(),
                Self::Close(tk) => tk.to_string(),
                Self::Literal(s) => s.to_owned(),
                Self::Expression(n) => format!("{} {n} {}", Kind::LeftBrace, Kind::RightBrace),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct InterpolatedString {
    pub nodes: Vec<InterpolatedStringPart>,
    pub token: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for InterpolatedString {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.nodes
            .iter()
            .flat_map(|n| n.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl SrcLinking for InterpolatedString {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for InterpolatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl From<InterpolatedString> for Node {
    fn from(val: InterpolatedString) -> Self {
        Node::Value(Value::InterpolatedString(val))
    }
}

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

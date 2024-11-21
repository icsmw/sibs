use asttree::*;
use lexer::Token;
use uuid::Uuid;

/// Represents the position of a content in the source code.
///
/// The `SrcLink` struct holds the starting and ending indices,
/// allowing for precise location tracking within the source code.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct SrcLink {
    /// The starting index (inclusive).
    pub from: usize,
    /// The ending index (exclusive).
    pub to: usize,
    /// The uuid of source code file
    pub src: Uuid,
}

impl SrcLink {
    /// Creates a new `SrcLink` with the specified starting and ending indices.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting index.
    /// * `to` - The ending index.
    /// * `src` - The uuid of source code file
    pub fn new(from: usize, to: usize, src: &Uuid) -> Self {
        Self {
            from,
            to,
            src: *src,
        }
    }
}

impl From<&Token> for SrcLink {
    fn from(token: &Token) -> Self {
        Self {
            from: token.pos.from,
            to: token.pos.to,
            src: token.src.to_owned(),
        }
    }
}

impl From<(&Token, &Token)> for SrcLink {
    fn from((from, to): (&Token, &Token)) -> Self {
        Self {
            from: from.pos.from,
            to: to.pos.to,
            src: from.src.to_owned(),
        }
    }
}

impl From<&Node> for SrcLink {
    fn from(node: &Node) -> Self {
        match node {
            Node::ControlFlowModifier(n) => n.into(),
            Node::Declaration(n) => n.into(),
            Node::Expression(n) => n.into(),
            Node::Miscellaneous(n) => n.into(),
            Node::Root(n) => n.into(),
            Node::Statement(n) => n.into(),
            Node::Value(n) => n.into(),
        }
    }
}

impl From<&Box<Node>> for SrcLink {
    fn from(node: &Box<Node>) -> Self {
        match node.as_ref() {
            Node::ControlFlowModifier(n) => n.into(),
            Node::Declaration(n) => n.into(),
            Node::Expression(n) => n.into(),
            Node::Miscellaneous(n) => n.into(),
            Node::Root(n) => n.into(),
            Node::Statement(n) => n.into(),
            Node::Value(n) => n.into(),
        }
    }
}

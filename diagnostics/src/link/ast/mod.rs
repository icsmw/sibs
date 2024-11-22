mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;
use asttree::*;

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

mod cfm;
mod declaration;
mod expression;
mod linking;
mod lookup;
mod metadata;
mod miscellaneous;
mod root;
mod statement;
mod targets;
mod value;

pub use cfm::*;
pub use declaration::*;
pub use expression::*;
pub use linking::*;
pub use lookup::*;
pub use metadata::*;
pub use miscellaneous::*;
pub use root::*;
pub use statement::*;
pub use targets::*;
pub use value::*;

pub(crate) use lexer::*;
use std::fmt;
pub(crate) use uuid::Uuid;

#[cfg(feature = "proptests")]
pub const PROPTEST_DEEP_FACTOR: u8 = 5;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
    Declaration(Declaration),
    Value(Value),
    ControlFlowModifier(ControlFlowModifier),
    Root(Root),
    Miscellaneous(Miscellaneous),
}

#[cfg(feature = "proptests")]
impl Default for Node {
    fn default() -> Self {
        Node::Miscellaneous(Miscellaneous::Comment(Comment {
            token: Token::for_test(Kind::Comment(String::from("DEFAULT NODE VALUE"))),
            uuid: Uuid::new_v4(),
        }))
    }
}

pub trait Identification {
    fn uuid(&self) -> &Uuid;
}

impl Identification for Node {
    fn uuid(&self) -> &Uuid {
        match self {
            Self::Statement(n) => n.uuid(),
            Self::Expression(n) => n.uuid(),
            Self::Declaration(n) => n.uuid(),
            Self::Value(n) => n.uuid(),
            Self::ControlFlowModifier(n) => n.uuid(),
            Self::Root(n) => n.uuid(),
            Self::Miscellaneous(n) => n.uuid(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinkedNode {
    pub node: Node,
    pub md: Metadata,
}

impl LinkedNode {
    pub fn from_node(node: Node) -> Self {
        LinkedNode {
            node,
            md: Metadata::default(),
        }
    }
}

impl Identification for LinkedNode {
    fn uuid(&self) -> &Uuid {
        self.node.uuid()
    }
}

impl fmt::Display for LinkedNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.md.meta_to_string(), self.node)
    }
}

impl SrcLinking for Node {
    fn link(&self) -> SrcLink {
        match self {
            Self::Statement(n) => n.link(),
            Self::Expression(n) => n.link(),
            Self::Declaration(n) => n.link(),
            Self::Value(n) => n.link(),
            Self::ControlFlowModifier(n) => n.link(),
            Self::Root(n) => n.link(),
            Self::Miscellaneous(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        match self {
            Self::Statement(n) => n.slink(),
            Self::Expression(n) => n.slink(),
            Self::Declaration(n) => n.slink(),
            Self::Value(n) => n.slink(),
            Self::ControlFlowModifier(n) => n.slink(),
            Self::Root(n) => n.slink(),
            Self::Miscellaneous(n) => n.slink(),
        }
    }
}

// impl Lookup for LinkedNode {
//     fn lookup(&self, trgs: &[NodeTarget]) -> Vec<FoundNode> {
//         self.node.lookup(trgs)
//     }
// }

// impl Lookup for &LinkedNode {
//     fn lookup(&self, trgs: &[NodeTarget]) -> Vec<FoundNode> {
//         self.node.lookup(trgs)
//     }
// }

// impl Lookup for Box<LinkedNode> {
//     fn lookup(&self, trgs: &[NodeTarget]) -> Vec<FoundNode> {
//         self.node.lookup(trgs)
//     }
// }

impl SrcLinking for LinkedNode {
    fn link(&self) -> SrcLink {
        self.node.link()
    }
    fn slink(&self) -> SrcLink {
        self.node.slink()
    }
}

impl SrcLinking for &LinkedNode {
    fn link(&self) -> SrcLink {
        self.node.link()
    }
    fn slink(&self) -> SrcLink {
        self.node.slink()
    }
}

impl SrcLinking for Box<LinkedNode> {
    fn link(&self) -> SrcLink {
        self.node.link()
    }
    fn slink(&self) -> SrcLink {
        self.node.slink()
    }
}

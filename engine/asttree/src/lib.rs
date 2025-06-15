mod cfm;
mod declaration;
mod diagnostic;
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
pub use diagnostic::*;
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

impl Diagnostic for Node {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        match self {
            Self::Statement(n) => n.located(src, pos),
            Self::Expression(n) => n.located(src, pos),
            Self::Declaration(n) => n.located(src, pos),
            Self::Value(n) => n.located(src, pos),
            Self::ControlFlowModifier(n) => n.located(src, pos),
            Self::Root(n) => n.located(src, pos),
            Self::Miscellaneous(n) => n.located(src, pos),
        }
    }
    fn get_position(&self) -> Position {
        match self {
            Self::Statement(n) => n.get_position(),
            Self::Expression(n) => n.get_position(),
            Self::Declaration(n) => n.get_position(),
            Self::Value(n) => n.get_position(),
            Self::ControlFlowModifier(n) => n.get_position(),
            Self::Root(n) => n.get_position(),
            Self::Miscellaneous(n) => n.get_position(),
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        match self {
            Self::Statement(n) => n.childs(),
            Self::Expression(n) => n.childs(),
            Self::Declaration(n) => n.childs(),
            Self::Value(n) => n.childs(),
            Self::ControlFlowModifier(n) => n.childs(),
            Self::Root(n) => n.childs(),
            Self::Miscellaneous(n) => n.childs(),
        }
    }
}

pub trait Identification {
    fn uuid(&self) -> &Uuid;
    fn ident(&self) -> String {
        String::new()
    }
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
    fn ident(&self) -> String {
        match self {
            Self::Statement(n) => n.ident(),
            Self::Expression(n) => n.ident(),
            Self::Declaration(n) => n.ident(),
            Self::Value(n) => n.ident(),
            Self::ControlFlowModifier(n) => n.ident(),
            Self::Root(n) => n.ident(),
            Self::Miscellaneous(n) => n.ident(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinkedNode {
    node: Node,
    md: Metadata,
}

impl LinkedNode {
    pub fn new(node: Node, md: Metadata) -> Self {
        Self { node, md }
    }
    pub fn from_node(node: Node) -> Self {
        Self {
            node,
            md: Metadata::default(),
        }
    }
    pub fn get_node(&self) -> &Node {
        &self.node
    }
    pub fn get_md(&self) -> &Metadata {
        &self.md
    }
    pub fn get_mut_node(&mut self) -> &mut Node {
        &mut self.node
    }
    pub fn get_mut_md(&mut self) -> &mut Metadata {
        &mut self.md
    }
}

impl Diagnostic for LinkedNode {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        self.node.located(src, pos)
    }
    fn get_position(&self) -> Position {
        self.node.get_position()
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        [self.md.childs(), self.node.childs()].concat()
    }
}

impl Identification for LinkedNode {
    fn uuid(&self) -> &Uuid {
        self.node.uuid()
    }
    fn ident(&self) -> String {
        self.node.ident()
    }
}

impl fmt::Display for LinkedNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.md.meta_to_string(),
            self.node,
            self.md.ppm_to_string()
        )
    }
}

impl Lookup<'_> for Node {
    fn lookup(&self, trgs: &[NodeTarget]) -> Vec<FoundNode> {
        match self {
            Self::Statement(n) => n.lookup(trgs),
            Self::Expression(n) => n.lookup(trgs),
            Self::Declaration(n) => n.lookup(trgs),
            Self::Value(n) => n.lookup(trgs),
            Self::ControlFlowModifier(n) => n.lookup(trgs),
            Self::Root(n) => n.lookup(trgs),
            Self::Miscellaneous(n) => n.lookup(trgs),
        }
    }
}

impl FindMutByUuid for Node {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            Self::Statement(n) => n.find_mut_by_uuid(uuid),
            Self::Expression(n) => n.find_mut_by_uuid(uuid),
            Self::Declaration(n) => n.find_mut_by_uuid(uuid),
            Self::Value(n) => n.find_mut_by_uuid(uuid),
            Self::ControlFlowModifier(n) => n.find_mut_by_uuid(uuid),
            Self::Root(n) => n.find_mut_by_uuid(uuid),
            Self::Miscellaneous(n) => n.find_mut_by_uuid(uuid),
        }
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

impl Lookup<'_> for LinkedNode {
    fn lookup(&self, trgs: &[NodeTarget]) -> Vec<FoundNode> {
        self.node.lookup(trgs)
    }
}

impl Lookup<'_> for Box<LinkedNode> {
    fn lookup(&self, trgs: &[NodeTarget]) -> Vec<FoundNode> {
        self.node.lookup(trgs)
    }
}

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

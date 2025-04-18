mod arg_assignation;
mod arg_assigned_value;
mod assignation;
mod assigned_value;
mod block;
mod r#break;
mod r#for;
mod r#if;
mod join;
mod r#loop;
mod oneof;
mod optional;
mod r#return;
mod r#while;

pub use arg_assignation::*;
pub use arg_assigned_value::*;
pub use assignation::*;
pub use assigned_value::*;
pub use block::*;
pub use join::*;
pub use oneof::*;
pub use optional::*;
pub use r#break::*;
pub use r#for::*;
pub use r#if::*;
pub use r#loop::*;
pub use r#return::*;
pub use r#while::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Statement {
    /// { ... }
    Block(Block),
    /// break;
    Break(Break),
    /// return;
    Return(Return),
    /// a > 5 => func();
    /// a < 4 || b > 5 => a = 5;
    /// (a < 100 || b > 100) && v != 0 => { ... };
    /// a > 5 => break;
    /// a > 5 => return;
    /// a > 5 => loop ...
    /// a > 5 => while ...
    /// a > 5 => for ...
    /// a > 5 => each ...
    /// a > 5 => oneof ...
    /// a > 5 => join ...
    Optional(Optional),
    /// if a > 4 { ... }
    /// if a > 5 { ... } if b > 5 { ... }
    /// if a > 5 { ... } if b > 5 { ... } else { ... }
    If(If),
    /// for (el, n) in 0..1 { ... };
    /// for (el, n) in from..to { ... };
    /// for (el, n) in elements { ... };
    /// for (el, n) in [1, 2, 3] { ... };
    /// for (el, n) in ["one", "two", "three"] { ... };
    /// for el in 0..1 { ... };
    /// for el in from..to { ... };
    /// for el in elements { ... };
    /// for el in [1, 2, 3] { ... };
    /// for el in ["one", "two", "three"] { ... };
    For(For),
    /// while s < 100 { ... };
    /// while a < 100 || b > 100 { ... };
    /// while (a < 100 || b > 100) && v != 0 { ... };
    While(While),
    /// loop { ... }
    Loop(Loop),
    /// a = 1, a = func(), etc.
    Assignation(Assignation),
    /// any value to assignate to variable
    AssignedValue(AssignedValue),
    /// a = 1, a = "text", etc.
    ArgumentAssignation(ArgumentAssignation),
    /// any value to assignate to argument
    ArgumentAssignedValue(ArgumentAssignedValue),
    OneOf(OneOf),
    /// join(`command`, `command`);
    Join(Join),
}

impl Statement {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::Assignation(n) => &n.uuid,
            Self::AssignedValue(n) => &n.uuid,
            Self::ArgumentAssignation(n) => &n.uuid,
            Self::ArgumentAssignedValue(n) => &n.uuid,
            Self::Block(n) => &n.uuid,
            Self::Break(n) => &n.uuid,
            Self::For(n) => &n.uuid,
            Self::If(n) => &n.uuid,
            Self::Join(n) => &n.uuid,
            Self::Loop(n) => &n.uuid,
            Self::OneOf(n) => &n.uuid,
            Self::Optional(n) => &n.uuid,
            Self::Return(n) => &n.uuid,
            Self::While(n) => &n.uuid,
        }
    }
}

impl<'a> Lookup<'a> for Statement {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            Self::Assignation(n) => n.lookup(trgs),
            Self::AssignedValue(n) => n.lookup(trgs),
            Self::ArgumentAssignation(n) => n.lookup(trgs),
            Self::ArgumentAssignedValue(n) => n.lookup(trgs),
            Self::Block(n) => n.lookup(trgs),
            Self::Break(n) => n.lookup(trgs),
            Self::For(n) => n.lookup(trgs),
            Self::If(n) => n.lookup(trgs),
            Self::Join(n) => n.lookup(trgs),
            Self::Loop(n) => n.lookup(trgs),
            Self::OneOf(n) => n.lookup(trgs),
            Self::Optional(n) => n.lookup(trgs),
            Self::Return(n) => n.lookup(trgs),
            Self::While(n) => n.lookup(trgs),
        }
    }
}

impl FindMutByUuid for Statement {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            Self::Assignation(n) => n.find_mut_by_uuid(uuid),
            Self::AssignedValue(n) => n.find_mut_by_uuid(uuid),
            Self::ArgumentAssignation(n) => n.find_mut_by_uuid(uuid),
            Self::ArgumentAssignedValue(n) => n.find_mut_by_uuid(uuid),
            Self::Block(n) => n.find_mut_by_uuid(uuid),
            Self::Break(n) => n.find_mut_by_uuid(uuid),
            Self::For(n) => n.find_mut_by_uuid(uuid),
            Self::If(n) => n.find_mut_by_uuid(uuid),
            Self::Join(n) => n.find_mut_by_uuid(uuid),
            Self::Loop(n) => n.find_mut_by_uuid(uuid),
            Self::OneOf(n) => n.find_mut_by_uuid(uuid),
            Self::Optional(n) => n.find_mut_by_uuid(uuid),
            Self::Return(n) => n.find_mut_by_uuid(uuid),
            Self::While(n) => n.find_mut_by_uuid(uuid),
        }
    }
}

impl SrcLinking for Statement {
    fn link(&self) -> SrcLink {
        match self {
            Self::Assignation(n) => n.link(),
            Self::AssignedValue(n) => n.link(),
            Self::ArgumentAssignation(n) => n.link(),
            Self::ArgumentAssignedValue(n) => n.link(),
            Self::Block(n) => n.link(),
            Self::Break(n) => n.link(),
            Self::For(n) => n.link(),
            Self::If(n) => n.link(),
            Self::Join(n) => n.link(),
            Self::Loop(n) => n.link(),
            Self::OneOf(n) => n.link(),
            Self::Optional(n) => n.link(),
            Self::Return(n) => n.link(),
            Self::While(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl From<Statement> for Node {
    fn from(val: Statement) -> Self {
        Node::Statement(val)
    }
}

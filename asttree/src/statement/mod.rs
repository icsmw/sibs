mod assignation;
mod assigned_value;
mod block;
mod r#break;
mod each;
mod r#for;
mod r#if;
mod join;
mod r#loop;
mod oneof;
mod optional;
mod r#return;
mod r#while;

pub use assignation::*;
pub use assigned_value::*;
pub use block::*;
pub use each::*;
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
    /// each(el, n, elements) { el; };
    /// each(el, n, [1, 2, 3]) { el; };
    Each(Each),
    /// a = 1, a = func(), etc.
    Assignation(Assignation),
    /// any value to assignate to variable
    AssignedValue(AssignedValue),
    OneOf(OneOf),
    Join(Join),
}

impl Statement {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::Assignation(n) => &n.uuid,
            Self::AssignedValue(n) => &n.uuid,
            Self::Block(n) => &n.uuid,
            Self::Break(n) => &n.uuid,
            Self::Each(n) => &n.uuid,
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
            Self::Block(n) => n.lookup(trgs),
            Self::Break(n) => n.lookup(trgs),
            Self::Each(n) => n.lookup(trgs),
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

impl SrcLinking for Statement {
    fn link(&self) -> SrcLink {
        match self {
            Self::Assignation(n) => n.link(),
            Self::AssignedValue(n) => n.link(),
            Self::Block(n) => n.link(),
            Self::Break(n) => n.link(),
            Self::Each(n) => n.link(),
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

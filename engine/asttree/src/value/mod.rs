mod array;
mod boolean;
mod closure;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

pub use array::*;
pub use boolean::*;
pub use closure::*;
pub use error::*;
pub use interpolated_string::*;
pub use number::*;
pub use primitive_string::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Value {
    Error(Error),
    Boolean(Boolean),
    Number(Number),
    Array(Array),
    InterpolatedString(InterpolatedString),
    PrimitiveString(PrimitiveString),
    Closure(Closure),
}

impl Identification for Value {
    fn uuid(&self) -> &Uuid {
        match self {
            Self::Error(n) => &n.uuid,
            Self::Boolean(n) => &n.uuid,
            Self::Number(n) => &n.uuid,
            Self::Array(n) => &n.uuid,
            Self::InterpolatedString(n) => &n.uuid,
            Self::PrimitiveString(n) => &n.uuid,
            Self::Closure(n) => &n.uuid,
        }
    }
    fn ident(&self) -> String {
        match self {
            Self::Error(..) => ValueId::Error.to_string(),
            Self::Boolean(..) => ValueId::Boolean.to_string(),
            Self::Number(..) => ValueId::Number.to_string(),
            Self::Array(..) => ValueId::Array.to_string(),
            Self::InterpolatedString(..) => ValueId::InterpolatedString.to_string(),
            Self::PrimitiveString(..) => ValueId::PrimitiveString.to_string(),
            Self::Closure(..) => ValueId::Closure.to_string(),
        }
    }
}

impl Diagnostic for Value {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        match self {
            Self::Error(n) => n.located(src, pos),
            Self::Boolean(n) => n.located(src, pos),
            Self::Number(n) => n.located(src, pos),
            Self::Array(n) => n.located(src, pos),
            Self::InterpolatedString(n) => n.located(src, pos),
            Self::PrimitiveString(n) => n.located(src, pos),
            Self::Closure(n) => n.located(src, pos),
        }
    }
    fn get_position(&self) -> Position {
        match self {
            Self::Error(n) => n.get_position(),
            Self::Boolean(n) => n.get_position(),
            Self::Number(n) => n.get_position(),
            Self::Array(n) => n.get_position(),
            Self::InterpolatedString(n) => n.get_position(),
            Self::PrimitiveString(n) => n.get_position(),
            Self::Closure(n) => n.get_position(),
        }
    }

    fn childs(&self) -> Vec<&LinkedNode> {
        match self {
            Self::Error(n) => n.childs(),
            Self::Boolean(n) => n.childs(),
            Self::Number(n) => n.childs(),
            Self::Array(n) => n.childs(),
            Self::InterpolatedString(n) => n.childs(),
            Self::PrimitiveString(n) => n.childs(),
            Self::Closure(n) => n.childs(),
        }
    }
}

impl<'a> Lookup<'a> for Value {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            Self::Error(n) => n.lookup(trgs),
            Self::Boolean(n) => n.lookup(trgs),
            Self::Number(n) => n.lookup(trgs),
            Self::Array(n) => n.lookup(trgs),
            Self::InterpolatedString(n) => n.lookup(trgs),
            Self::PrimitiveString(n) => n.lookup(trgs),
            Self::Closure(n) => n.lookup(trgs),
        }
    }
}

impl FindMutByUuid for Value {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            Self::Error(n) => n.find_mut_by_uuid(uuid),
            Self::Boolean(n) => n.find_mut_by_uuid(uuid),
            Self::Number(n) => n.find_mut_by_uuid(uuid),
            Self::Array(n) => n.find_mut_by_uuid(uuid),
            Self::InterpolatedString(n) => n.find_mut_by_uuid(uuid),
            Self::PrimitiveString(n) => n.find_mut_by_uuid(uuid),
            Self::Closure(n) => n.find_mut_by_uuid(uuid),
        }
    }
}

impl SrcLinking for Value {
    fn link(&self) -> SrcLink {
        match self {
            Self::Error(n) => n.link(),
            Self::Boolean(n) => n.link(),
            Self::Number(n) => n.link(),
            Self::Array(n) => n.link(),
            Self::InterpolatedString(n) => n.link(),
            Self::PrimitiveString(n) => n.link(),
            Self::Closure(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl From<Value> for Node {
    fn from(val: Value) -> Self {
        Node::Value(val)
    }
}

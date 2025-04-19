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

impl Value {
    pub fn uuid(&self) -> &Uuid {
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

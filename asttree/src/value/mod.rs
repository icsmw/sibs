mod array;
mod boolean;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

pub use array::*;
pub use boolean::*;
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
        }
    }
}

mod conflict;
mod interest;
mod read;

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

use std::fmt;

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

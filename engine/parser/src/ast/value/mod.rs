mod conflict;

mod array;
mod boolean;
mod closure;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;

impl AsVec<ValueId> for ValueId {
    fn as_vec() -> Vec<ValueId> {
        ValueId::as_vec()
    }
}

impl TryRead<Value, ValueId> for Value {
    fn try_read(parser: &Parser, id: ValueId) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(match id {
            ValueId::PrimitiveString => PrimitiveString::read_as_linked(parser)?,
            ValueId::InterpolatedString => InterpolatedString::read_as_linked(parser)?,
            ValueId::Boolean => Boolean::read_as_linked(parser)?,
            ValueId::Number => Number::read_as_linked(parser)?,
            ValueId::Array => Array::read_as_linked(parser)?,
            ValueId::Error => Error::read_as_linked(parser)?,
            ValueId::Closure => Closure::read_as_linked(parser)?,
        })
    }
}

use crate::*;

impl AsVec<ValueId> for ValueId {
    fn as_vec() -> Vec<ValueId> {
        ValueId::as_vec()
    }
}

impl Read<Value, ValueId> for Value {}

impl TryRead<Value, ValueId> for Value {
    fn try_read(parser: &mut Parser, id: ValueId) -> Result<Option<Value>, LinkedErr<E>> {
        Ok(match id {
            ValueId::PrimitiveString => PrimitiveString::read(parser)?.map(Value::PrimitiveString),
            ValueId::InterpolatedString => {
                InterpolatedString::read(parser)?.map(Value::InterpolatedString)
            }
            ValueId::Boolean => Boolean::read(parser)?.map(Value::Boolean),
            ValueId::Number => Number::read(parser)?.map(Value::Number),
            ValueId::Array => Array::read(parser)?.map(Value::Array),
            ValueId::Error => Error::read(parser)?.map(Value::Error),
        })
    }
}

use crate::*;

impl AsVec<ValueId> for ValueId {
    fn as_vec() -> Vec<ValueId> {
        ValueId::as_vec()
    }
}

impl Read<Value, ValueId> for Value {}

impl TryRead<Value, ValueId> for Value {
    fn try_read(parser: &mut Parser, id: ValueId, nodes: &Nodes) -> Result<Option<Value>, E> {
        Ok(match id {
            ValueId::PrimitiveString => {
                PrimitiveString::read(parser, nodes)?.map(Value::PrimitiveString)
            }
            ValueId::InterpolatedString => {
                InterpolatedString::read(parser, nodes)?.map(Value::InterpolatedString)
            }
            ValueId::Boolean => Boolean::read(parser, nodes)?.map(Value::Boolean),
            ValueId::Number => Number::read(parser, nodes)?.map(Value::Number),
            ValueId::Array => Array::read(parser, nodes)?.map(Value::Array),
            ValueId::Error => Error::read(parser, nodes)?.map(Value::Error),
        })
    }
}

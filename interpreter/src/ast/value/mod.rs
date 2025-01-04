mod array;
mod boolean;
mod closure;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;

impl Interpret for Value {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Value::Array(n) => n.interpret(rt),
            Value::Boolean(n) => n.interpret(rt),
            Value::Error(n) => n.interpret(rt),
            Value::InterpolatedString(n) => n.interpret(rt),
            Value::Number(n) => n.interpret(rt),
            Value::PrimitiveString(n) => n.interpret(rt),
            Value::Closure(n) => n.interpret(rt),
        }
    }
}

mod array;
mod boolean;
mod closure;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;

impl Interpret for Value {
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Value::Array(n) => n.interpret(rt, cx),
            Value::Boolean(n) => n.interpret(rt, cx),
            Value::Error(n) => n.interpret(rt, cx),
            Value::InterpolatedString(n) => n.interpret(rt, cx),
            Value::Number(n) => n.interpret(rt, cx),
            Value::PrimitiveString(n) => n.interpret(rt, cx),
            Value::Closure(n) => n.interpret(rt, cx),
        }
    }
}

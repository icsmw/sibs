mod array;
mod boolean;
mod closure;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;

impl Interpret for Value {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Value::Array(n) => n.interpret(env),
            Value::Boolean(n) => n.interpret(env),
            Value::Error(n) => n.interpret(env),
            Value::InterpolatedString(n) => n.interpret(env),
            Value::Number(n) => n.interpret(env),
            Value::PrimitiveString(n) => n.interpret(env),
            Value::Closure(n) => n.interpret(env),
        }
    }
}

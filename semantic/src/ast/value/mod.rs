mod array;
mod boolean;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;

impl InferType for Value {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            Value::Array(n) => n.infer_type(scx),
            Value::Boolean(n) => n.infer_type(scx),
            Value::Error(n) => n.infer_type(scx),
            Value::InterpolatedString(n) => n.infer_type(scx),
            Value::Number(n) => n.infer_type(scx),
            Value::PrimitiveString(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for Value {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Value::Array(n) => n.initialize(scx),
            Value::Boolean(n) => n.initialize(scx),
            Value::Error(n) => n.initialize(scx),
            Value::InterpolatedString(n) => n.initialize(scx),
            Value::Number(n) => n.initialize(scx),
            Value::PrimitiveString(n) => n.initialize(scx),
        }
    }
}

impl Finalization for Value {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Value::Array(n) => n.finalize(scx),
            Value::Boolean(n) => n.finalize(scx),
            Value::Error(n) => n.finalize(scx),
            Value::InterpolatedString(n) => n.finalize(scx),
            Value::Number(n) => n.finalize(scx),
            Value::PrimitiveString(n) => n.finalize(scx),
        }
    }
}

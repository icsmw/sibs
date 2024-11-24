mod array;
mod boolean;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;

impl InferType for Value {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Value::Array(n) => n.infer_type(tcx),
            Value::Boolean(n) => n.infer_type(tcx),
            Value::Error(n) => n.infer_type(tcx),
            Value::InterpolatedString(n) => n.infer_type(tcx),
            Value::Number(n) => n.infer_type(tcx),
            Value::PrimitiveString(n) => n.infer_type(tcx),
        }
    }
}

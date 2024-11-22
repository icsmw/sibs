mod array;
mod boolean;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;
use asttree::*;

impl From<&Value> for SrcLink {
    fn from(node: &Value) -> Self {
        match node {
            Value::Error(n) => n.into(),
            Value::Boolean(n) => n.into(),
            Value::Number(n) => n.into(),
            Value::Array(n) => n.into(),
            Value::InterpolatedString(n) => n.into(),
            Value::PrimitiveString(n) => n.into(),
        }
    }
}

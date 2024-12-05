#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Number {
    pub inner: f64,
    pub token: Token,
    pub uuid: Uuid,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<Number> for Node {
    fn from(val: Number) -> Self {
        Node::Value(Value::Number(val))
    }
}

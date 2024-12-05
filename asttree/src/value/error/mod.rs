#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub token: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
    pub open: Token,
    pub close: Token,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token, self.open, self.node, self.close
        )
    }
}

impl From<Error> for Node {
    fn from(val: Error) -> Self {
        Node::Value(Value::Error(val))
    }
}

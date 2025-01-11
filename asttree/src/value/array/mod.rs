#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Array {
    pub open: Token,
    pub els: Vec<LinkedNode>,
    pub close: Token,
    pub uuid: Uuid,
}

impl SrcLinking for Array {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.open, &self.close)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.open,
            self.els
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            self.close
        )
    }
}

impl From<Array> for Node {
    fn from(val: Array) -> Self {
        Node::Value(Value::Array(val))
    }
}

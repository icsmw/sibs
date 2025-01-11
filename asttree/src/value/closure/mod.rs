#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Closure {
    pub args: Vec<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl SrcLinking for Closure {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.open, &self.block)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.open,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            self.close,
            self.block
        )
    }
}

impl From<Closure> for Node {
    fn from(val: Closure) -> Self {
        Node::Value(Value::Closure(val))
    }
}

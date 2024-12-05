#[cfg(feature = "proptests")]
mod proptests;
use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Meta {
    pub token: Token,
    pub uuid: Uuid,
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", Kind::LF, self.token, Kind::LF)
    }
}

impl From<Meta> for Node {
    fn from(val: Meta) -> Self {
        Node::Miscellaneous(Miscellaneous::Meta(val))
    }
}

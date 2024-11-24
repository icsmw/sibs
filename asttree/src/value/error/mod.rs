#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub token: Token,
    pub node: Box<Node>,
    pub uuid: Uuid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token,
            Kind::LeftParen,
            self.node,
            Kind::RightParen
        )
    }
}

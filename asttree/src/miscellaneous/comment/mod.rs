#[cfg(feature = "proptests")]
mod proptests;

use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Comment {
    pub token: Token,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", Kind::LF, self.token, Kind::LF)
    }
}

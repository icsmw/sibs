mod link;
#[cfg(test)]
mod proptests;
mod read;

use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Comment {
    token: Token,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", Kind::LF, self.token, Kind::LF)
    }
}

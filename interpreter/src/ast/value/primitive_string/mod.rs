#[cfg(test)]
mod proptests;
mod read;
mod link;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PrimitiveString {
    pub inner: String,
    pub token: Token,
}

impl fmt::Display for PrimitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

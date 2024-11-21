#[cfg(feature = "proptests")]
mod proptests;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Number {
    pub inner: f64,
    pub token: Token,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

mod read;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub inner: bool,
    pub token: Token,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

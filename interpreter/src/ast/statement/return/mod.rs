#[cfg(test)]
mod proptests;
mod read;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Return {
    token: Token,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

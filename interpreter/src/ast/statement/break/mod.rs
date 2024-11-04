mod read;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Break {
    token: Token,
}

impl fmt::Display for Break {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

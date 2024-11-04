use lexer::Token;

#[cfg(test)]
mod proptests;
mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Variable {
    pub ident: String,
    pub token: Token,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

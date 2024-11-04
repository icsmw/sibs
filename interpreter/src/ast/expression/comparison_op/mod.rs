use lexer::Token;

mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    BangEqual,
}

#[derive(Debug, Clone)]
pub struct ComparisonOp {
    pub token: Token,
    pub operator: ComparisonOperator,
}

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

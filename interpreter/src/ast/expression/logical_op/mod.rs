mod link;
#[cfg(test)]
mod proptests;
mod read;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, Clone)]
pub struct LogicalOp {
    pub token: Token,
    pub operator: LogicalOperator,
}

impl fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

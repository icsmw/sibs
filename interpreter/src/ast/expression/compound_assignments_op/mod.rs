mod read;

use lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CompoundAssignmentsOperator {
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
}

#[derive(Debug, Clone)]
pub struct CompoundAssignmentsOp {
    token: Token,
    operator: CompoundAssignmentsOperator,
}

impl fmt::Display for CompoundAssignmentsOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

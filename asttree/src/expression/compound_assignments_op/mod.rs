#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CompoundAssignmentsOperator {
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
}

impl fmt::Display for CompoundAssignmentsOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CompoundAssignmentsOperator::PlusEqual => Kind::PlusEqual,
                CompoundAssignmentsOperator::MinusEqual => Kind::MinusEqual,
                CompoundAssignmentsOperator::StarEqual => Kind::StarEqual,
                CompoundAssignmentsOperator::SlashEqual => Kind::SlashEqual,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct CompoundAssignmentsOp {
    pub token: Token,
    pub operator: CompoundAssignmentsOperator,
    pub uuid: Uuid,
}

impl fmt::Display for CompoundAssignmentsOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.operator)
    }
}

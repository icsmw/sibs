use lexer::KindId;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Next nodes are in conflict: {0}")]
    NodesAreInConflict(String),
    #[error("No closing: {0}")]
    NoClosing(KindId),
    #[error("Unexpected logical operator: {0}")]
    UnexpectedLogicalOperator(KindId),
    #[error("Unexpected binary operator: {0}")]
    UnexpectedBinaryOperator(KindId),
    #[error("Missed logical operator && or ||")]
    MissedLogicalOperator,
    #[error("Missed binary operator -, +, *, /")]
    MissedBinaryOperator,
    #[error("Missed comma")]
    MissedComma,
    #[error("Infinite number cannot be used")]
    InfiniteNumber,
    #[error("Invalid right side of assignation: {0}")]
    InvalidAssignation(String),
    #[error("Error message is missed")]
    MissedErrorMessage,
    #[error("Unrecognized code: {0}")]
    UnrecognizedCode(String),
    #[error("Fail to parse string: {0}")]
    InvalidString(String),
    #[error("String has empty expression")]
    EmptyStringExpression,
    #[error("Not supported string injection in: {0}")]
    NotSupportedStringInjection(String),
    #[error("After {0} expected block")]
    NoExpectedBlockAfter(KindId),
    #[error("After {0} expected {1}, but not found")]
    MissedExpectation(String, String),
}

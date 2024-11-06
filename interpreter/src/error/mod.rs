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
    #[error("Missed logical operator && or ||")]
    MissedLogicalOperator,
    #[error("Infinite number cannot be used")]
    InfiniteNumber,
    #[error("After {0} expected block")]
    NoExpectedBlockAfter(KindId),
    #[error("After {0} expected {1}, but not found")]
    MissedExpectation(String, String),
}

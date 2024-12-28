use crate::*;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Attempt to leave global scope")]
    AttemptToLeaveGlobalScope,
    #[error("Attempt to set type without scope")]
    NoCurrentScope,
    #[error("Variable \"{0}\" isn't found")]
    ScopeNotFound(Uuid),
    #[error("Fail to receive message")]
    RecvError,
    #[error("Fail to send message")]
    SendError,
    #[error("Fail extract value")]
    FailExtractValue,
    #[error("Invalid value type; expected \"{0}\"")]
    InvalidValueType(String),
    #[error("Value type cannot be cast to public DataType")]
    NotPublicValueType,
    #[error("Some values cannot be converted into string")]
    CannotBeConvertedToString,
    #[error("Missed binary operator")]
    MissedBinaryOperator,
    #[error("Runtime error: {0}")]
    RuntimeError(RtError),
    #[error("Value cannot be compared")]
    NotComparableValue,
    #[error("Values cannot be compared, because of different type")]
    DifferentTypeOfValues,
    #[error("Invalid ComparisonSeq; cannot get the first value")]
    InvalidComparisonSeq,
    #[error("Invalid If statement; cannot get the final condition")]
    InvalidIfStatement,
    #[error("Fail to infer type")]
    FailInferType,
    #[error("Unexpected node: {0}")]
    UnexpectedNode(NodeId),
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
}

impl From<RtError> for E {
    fn from(err: RtError) -> Self {
        Self::RuntimeError(err)
    }
}

impl From<oneshot::error::RecvError> for E {
    fn from(_: oneshot::error::RecvError) -> Self {
        E::RecvError
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for E {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        E::SendError
    }
}

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
    #[error("Attempt to leave root scope's level")]
    AttemptToLeaveRootScopeLevel,
    #[error("Attempt to set type without root scope's level")]
    NoCurrentScopeLevel,
    #[error("No root scope found")]
    NoRootScope,
    #[error("Fail to find scope {0}")]
    FailToFindScope(Uuid),

    #[error("Function \"{0}\" has been registred already")]
    FuncAlreadyRegistered(String),
    #[error("Function \"{0}\" not found")]
    FuncNotFound(String),
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

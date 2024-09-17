use crate::{
    error::LinkedErr,
    inf::{context, operator, store, TokenGetter},
};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("{0}")]
    Context(String),
    #[error("{0}")]
    TryExecute(String),
    #[error("Function \"{0}\" executing error: {1}")]
    FunctionExecuting(String, String),
    #[error("Fail convert value to: {0}")]
    Converting(String),
    #[error("Invalid function's argument: {0}")]
    InvalidFunctionArg(String),
    #[error("Invalid arguments length; required: {0}; gotten: {1}")]
    InvalidArgumentsCount(String, String),
    #[error("Invalid closure arguments count; required: {0}; gotten: {1}")]
    InvalidClosureArgumentsCount(usize, usize),
    #[error("Closure's variable \"{0}\" is used outside of closure")]
    ClosureVariableIsUsed(String),
    #[error("Fail to extract bool value from closure output")]
    FailToExtractBoolValueFromClosure,
    #[error("IO error: {0}")]
    IO(String),
    #[error("SystemTimeError error: {0}")]
    SystemTimeError(String),
    #[error("VarError error: {0}")]
    VarError(String),
    #[error("Function \"{0}\" not exists")]
    FunctionNotExists(String),
    #[error("For \"{0}\" has been found multiple functions")]
    MultipleFunctionHasBeenFound(String),
    #[error("Fail to receive channel message: {0}")]
    RecvError(String),
    #[error("Fail to send channel message: {0}")]
    SendError(String),
    #[error("Error on executing \"{0}\": {1}")]
    Executing(String, String),
    #[error("{0}")]
    Store(store::E),
    #[error("{0}")]
    Other(String),
}

impl E {
    pub fn by(self, operator: &dyn TokenGetter) -> LinkedErr<E> {
        LinkedErr::new(self, Some(operator.token()))
    }
    pub fn linked(self, token: &usize) -> LinkedErr<E> {
        LinkedErr::new(self, Some(*token))
    }
    pub fn unlinked(self) -> LinkedErr<E> {
        LinkedErr::new(self, None)
    }
}

impl From<context::E> for LinkedErr<E> {
    fn from(err: context::E) -> Self {
        LinkedErr::unlinked(err.into())
    }
}

impl From<E> for LinkedErr<E> {
    fn from(err: E) -> Self {
        LinkedErr::unlinked(err)
    }
}

impl From<fshasher::E> for LinkedErr<E> {
    fn from(err: fshasher::E) -> Self {
        LinkedErr::unlinked(err.into())
    }
}

impl From<bstorage::E> for LinkedErr<E> {
    fn from(err: bstorage::E) -> Self {
        LinkedErr::unlinked(err.into())
    }
}

impl From<operator::E> for LinkedErr<E> {
    fn from(err: operator::E) -> Self {
        LinkedErr::unlinked(err.into())
    }
}

impl From<oneshot::error::RecvError> for LinkedErr<E> {
    fn from(value: oneshot::error::RecvError) -> Self {
        LinkedErr::unlinked(E::RecvError(value.to_string()))
    }
}

impl<T> From<mpsc::error::SendError<T>> for LinkedErr<E> {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        LinkedErr::unlinked(E::SendError(value.to_string()))
    }
}

// impl From<LinkedErr<context::E>> for LinkedErr<E> {
//     fn from(err: LinkedErr<context::E>) -> Self {
//         LinkedErr::new(err.e.into(), err.token)
//     }
// }
impl From<store::E> for E {
    fn from(e: store::E) -> Self {
        E::Store(e)
    }
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        E::Context(e.to_string())
    }
}

impl From<operator::E> for E {
    fn from(e: operator::E) -> Self {
        E::TryExecute(e.to_string())
    }
}

impl From<LinkedErr<operator::E>> for LinkedErr<E> {
    fn from(err: LinkedErr<operator::E>) -> Self {
        LinkedErr::new(err.e.into(), err.token)
    }
}

impl From<std::io::Error> for E {
    fn from(e: std::io::Error) -> Self {
        E::IO(e.to_string())
    }
}

impl From<std::time::SystemTimeError> for E {
    fn from(e: std::time::SystemTimeError) -> Self {
        E::SystemTimeError(e.to_string())
    }
}

impl From<std::env::VarError> for E {
    fn from(e: std::env::VarError) -> Self {
        E::VarError(e.to_string())
    }
}

impl From<oneshot::error::RecvError> for E {
    fn from(value: oneshot::error::RecvError) -> Self {
        E::RecvError(value.to_string())
    }
}
impl<T> From<mpsc::error::SendError<T>> for E {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        E::SendError(value.to_string())
    }
}

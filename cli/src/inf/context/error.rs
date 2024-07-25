use crate::{functions, inf::context::scenario, reader};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("{0}")]
    ScenarionError(scenario::E),
    #[error("{0}")]
    FunctionsError(functions::E),
    #[error("{0}")]
    ReaderError(String),
    #[error("IO error: {0}")]
    IO(String),
    #[error("Fail to receive channel message: {0}")]
    RecvError(String),
    #[error("Fail to send channel message: {0}")]
    SendError(String),
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
    }
}

impl From<scenario::E> for E {
    fn from(e: scenario::E) -> Self {
        E::ScenarionError(e)
    }
}

impl From<functions::E> for E {
    fn from(e: functions::E) -> Self {
        E::FunctionsError(e)
    }
}

impl From<reader::E> for E {
    fn from(e: reader::E) -> Self {
        E::ReaderError(e.to_string())
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

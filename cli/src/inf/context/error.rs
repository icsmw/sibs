use std::{io, path::PathBuf};

use crate::{executors, inf::scenario, reader};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug)]
pub enum E {
    #[error("No parent folder for: {0}")]
    NoParentFolderFor(PathBuf),
    #[error("Scenario error: {0}")]
    ScenarionError(scenario::E),
    #[error("Executors error: {0}")]
    ExecutorsError(executors::E),
    #[error("Reader error: {0}")]
    ReaderError(String),
    #[error("Fail register function \"{0}\" because it's already exists")]
    FunctionAlreadyExists(String),
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Fail to receive channel message: {0}")]
    RecvError(String),
    #[error("Fail to send channel message: {0}")]
    SendError(String),
}

impl From<scenario::E> for E {
    fn from(e: scenario::E) -> Self {
        E::ScenarionError(e)
    }
}

impl From<executors::E> for E {
    fn from(e: executors::E) -> Self {
        E::ExecutorsError(e)
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

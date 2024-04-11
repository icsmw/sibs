use std::{io, path::PathBuf};

use crate::{
    executors,
    inf::{map, scenario},
    reader,
};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug)]
pub enum E {
    #[error("Token {0} not found")]
    TokenNotFound(usize),
    #[error("Token {0} has invalid range; string len={1}; range [{2},{3}]")]
    TokenHasInvalidRange(usize, usize, usize, usize),
    #[error("File {0} already has a map")]
    FileAlreadyHasMap(PathBuf),
    #[error("Fail to find a token {0}")]
    FailToFindToken(usize),
    #[error("Fail to receive channel message: {0}")]
    RecvError(String),
    #[error("Fail to send channel message: {0}")]
    SendError(String),
    #[error("Map error: {0}")]
    MapError(map::E),
}

impl From<map::E> for E {
    fn from(e: map::E) -> Self {
        E::MapError(e)
    }
}

// impl From<executors::E> for E {
//     fn from(e: executors::E) -> Self {
//         E::ExecutorsError(e)
//     }
// }

// impl From<reader::E> for E {
//     fn from(e: reader::E) -> Self {
//         E::ReaderError(e.to_string())
//     }
// }

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

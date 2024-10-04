use crate::inf::tracker;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("IO error: {0}")]
    IO(String),
    #[error("Fail to setup command: {0}")]
    Setup(String),
    #[error("{0}")]
    TrackerError(tracker::E),
}

impl From<tracker::E> for E {
    fn from(e: tracker::E) -> Self {
        Self::TrackerError(e)
    }
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
    }
}

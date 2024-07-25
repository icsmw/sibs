use crate::inf::tracker;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("IO error: {0}")]
    IO(String),
    #[error("Fail to setup command")]
    Setup(String),
    #[error("Error on executing \"{0}\": {1}")]
    Executing(String, String),
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

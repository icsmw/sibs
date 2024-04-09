use crate::inf::tracker;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Fail to setup command")]
    Setup(String),
    #[error("Error on executing \"{0}\": {1}")]
    Executing(String, String),
    #[error("Tracker error {0}")]
    TrackerError(tracker::E),
}

impl From<tracker::E> for E {
    fn from(e: tracker::E) -> Self {
        Self::TrackerError(e)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Fail to get exist status of spawned command")]
    NoExitStatus
}

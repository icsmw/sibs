use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("IO error: {0}")]
    IO(String),
    #[error("Fail to shutdown journal")]
    ShutdownFail,
}

impl From<std::io::Error> for E {
    fn from(e: std::io::Error) -> Self {
        E::IO(e.to_string())
    }
}

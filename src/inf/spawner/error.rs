use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Fail to setup command")]
    Setup(String),
    #[error("Error on executing \"{0}\": {1}")]
    Executing(String, String),
}

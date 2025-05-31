use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Path doesn't exist: {0}")]
    PathDoesNotExist(String),
    #[error("Cannot find scenario file in current location: {0}")]
    ScenarioNotFound(String),
    #[error("IO error: {0}")]
    IO(String),
    #[error("Fail to get cwd from \"{0}\"")]
    FailToGetCwd(String),
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
    }
}

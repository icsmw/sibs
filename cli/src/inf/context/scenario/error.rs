use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("IO error: {0}")]
    IO(String),
    #[error("No parent folder for: {0}")]
    NoParentFolderFor(String),
    #[error("Path doesn't exist; path: {0}")]
    PathDoesNotExist(String),
    #[error("Fail to find any sibs files; default sibs file - build.sibs also wasn't found")]
    ScenarioNotFound,
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
    }
}

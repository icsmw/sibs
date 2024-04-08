use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("No parent folder for: {0}")]
    NoParentFolderFor(String),
    #[error("Not absolute path; path: {0}")]
    IsNotAbsolutePath(String),
    #[error("Path doesn't exist; path: {0}")]
    PathDoesNotExist(String),
    #[error("Fail to find any sibs files; default sibs file - build.sibs also wasn't found")]
    ScenarioNotFound,
}

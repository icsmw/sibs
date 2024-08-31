use crate::functions::E;
use std::path::PathBuf;
use thiserror::Error;

pub const NAME: &str = "import";

#[derive(Error, Debug)]
pub enum Error {
    #[error("File {0} doesn't exist")]
    NoFile(String),
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E::FunctionExecuting(NAME.to_string(), e.to_string())
    }
}

pub fn get(mut path: PathBuf, cwd: PathBuf) -> Result<PathBuf, E> {
    if path.is_relative() {
        path = cwd.join(path);
    }
    if !path.exists() {
        Err(Error::NoFile(path.to_string_lossy().to_string()))?;
    }
    Ok(path)
}

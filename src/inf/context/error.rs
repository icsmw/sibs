use crate::{executors, inf::scenario, reader};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("No parent folder for: {0}")]
    NoParentFolderFor(String),
    #[error("Scenario error: {0}")]
    ScenarionError(scenario::E),
    #[error("Executors error: {0}")]
    ExecutorsError(executors::E),
    #[error("Reader error: {0}")]
    ReaderError(reader::E),
    #[error("Fail register function \"{0}\" because it's already exists")]
    FunctionAlreadyExists(String),
}

impl From<scenario::E> for E {
    fn from(e: scenario::E) -> Self {
        E::ScenarionError(e)
    }
}

impl From<executors::E> for E {
    fn from(e: executors::E) -> Self {
        E::ExecutorsError(e)
    }
}

impl From<reader::E> for E {
    fn from(e: reader::E) -> Self {
        E::ReaderError(e)
    }
}

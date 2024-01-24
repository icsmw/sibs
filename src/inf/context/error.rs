use crate::{executors, inf::scenario};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("No parent folder for: {0}")]
    NoParentFolderFor(String),
    #[error("Scenario error: {0}")]
    ScenarionError(String),
    #[error("Executors error: {0}")]
    ExecutorsError(String),
    #[error("Fail register function \"{0}\" because it's already exists")]
    FunctionAlreadyExists(String),
}

impl From<scenario::E> for E {
    fn from(value: scenario::E) -> Self {
        E::ScenarionError(value.to_string())
    }
}

impl From<executors::E> for E {
    fn from(value: executors::E) -> Self {
        E::ExecutorsError(value.to_string())
    }
}

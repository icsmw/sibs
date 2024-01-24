use crate::inf::{context, operator};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Scenario error: {0}")]
    ContextError(String),
    #[error("Operator error: {0}")]
    OperatorError(String),
    #[error("Fail register function \"{0}\" because it's already exists")]
    FunctionAlreadyExists(String),
    #[error("Function \"{0}\" executing error: {1}")]
    FunctionExecutingError(String, String),
}

impl From<context::E> for E {
    fn from(value: context::E) -> Self {
        E::ContextError(value.to_string())
    }
}

impl From<operator::E> for E {
    fn from(value: operator::E) -> Self {
        E::OperatorError(value.to_string())
    }
}

use crate::inf::{context, operator};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Scenario error: {0}")]
    Context(String),
    #[error("Operator error: {0}")]
    Operator(String),
    #[error("Function \"{0}\" executing error: {1}")]
    FunctionExecuting(String, String),
}

impl From<context::E> for E {
    fn from(value: context::E) -> Self {
        E::Context(value.to_string())
    }
}

impl From<operator::E> for E {
    fn from(value: operator::E) -> Self {
        E::Operator(value.to_string())
    }
}

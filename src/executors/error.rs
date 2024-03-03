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
    #[error("Fail convert value to: {0}")]
    Converting(String),
    #[error("Invalid arguments length; required: {0}; gotten: {1}")]
    InvalidArgumentsCount(String, String),
    #[error("IO error: {0}")]
    IO(String),
    #[error("SystemTimeError error: {0}")]
    SystemTimeError(String),
    #[error("VarError error: {0}")]
    VarError(String),
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        E::Context(e.to_string())
    }
}

impl From<operator::E> for E {
    fn from(e: operator::E) -> Self {
        E::Operator(e.to_string())
    }
}

impl From<std::io::Error> for E {
    fn from(e: std::io::Error) -> Self {
        E::IO(e.to_string())
    }
}

impl From<std::time::SystemTimeError> for E {
    fn from(e: std::time::SystemTimeError) -> Self {
        E::SystemTimeError(e.to_string())
    }
}

impl From<std::env::VarError> for E {
    fn from(e: std::env::VarError) -> Self {
        E::VarError(e.to_string())
    }
}

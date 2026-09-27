use diagnostics::LinkedErr;
use enum_ids::enum_ids;
use parser::ParserError;
use runtime::{error::E as RtError, RtValue};
use thiserror::Error;

#[derive(Error, Debug)]
#[enum_ids(derive = "Debug")]
pub enum ExecutorError {
    #[error("Runtime setup error: {0}")]
    RuntimeSetup(RtError),

    #[error("Runtime shutdown error: {0}")]
    RuntimeShutdown(RtError),

    #[error("Execution failed: {0}")]
    Execution(LinkedErr<RtError>),

    #[error("Invalid context for execution")]
    InvalidContext,

    #[error("IO error: {0}")]
    IO(String),

    #[error("Parser error: {0}")]
    Parser(ParserError),

    #[error("Execution finished successfully, but runtime shutdown failed: {value:?}; {err}")]
    ValueAndShutdown { value: RtValue, err: RtError },

    #[error("{err}; runtime shutdown also failed: {shutdown_err}")]
    ErrorAndShutdown {
        #[source]
        err: Box<ExecutorError>,
        shutdown_err: RtError,
    },
}

impl From<RtError> for ExecutorError {
    fn from(err: RtError) -> Self {
        Self::RuntimeSetup(err)
    }
}

impl From<std::io::Error> for ExecutorError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.to_string())
    }
}

impl From<ParserError> for ExecutorError {
    fn from(err: ParserError) -> Self {
        Self::Parser(err)
    }
}

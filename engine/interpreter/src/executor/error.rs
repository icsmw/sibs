use diagnostics::LinkedErr;
use enum_ids::enum_ids;
use parser::Parser;
use parser::ParserError;
use runtime::{error::E as RtError, RtValue};
use thiserror::Error;

#[derive(Debug)]
pub struct ExecutionFailure {
    parser: Parser,
    err: LinkedErr<RtError>,
}

impl ExecutionFailure {
    pub(crate) fn new(parser: Parser, err: LinkedErr<RtError>) -> Self {
        Self { parser, err }
    }

    pub fn report(&self) -> Result<String, ExecutorError> {
        Ok(self.parser.report_err(&self.err)?)
    }

    pub fn inner(&self) -> &LinkedErr<RtError> {
        &self.err
    }
}

impl std::fmt::Display for ExecutionFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err.e)
    }
}

#[derive(Error, Debug)]
#[enum_ids(derive = "Debug")]
pub enum ExecutorError {
    #[error("Runtime setup error: {0}")]
    RuntimeSetup(RtError),

    #[error("Runtime shutdown error: {0}")]
    RuntimeShutdown(RtError),

    #[error("Execution failed: {0}")]
    Execution(Box<ExecutionFailure>),

    #[error("IO error: {0}")]
    IO(String),

    #[error("Parser error: {0}")]
    Parser(ParserError),

    #[error("Execution finished successfully, but runtime shutdown failed: {value:?}; {err}")]
    ValueAndShutdown { value: RtValue, err: RtError },
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

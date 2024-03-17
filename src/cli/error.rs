use crate::{
    error,
    error::LinkedErr,
    inf::{context, operator, scenario},
    reader,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("--scenario requires a path to *.sibs file")]
    NoPathToScenarioFile,
    #[error("Invalid request; expecting addition arguments after: {0}")]
    InvalidRequestAfter(String),
    #[error("No any options/commands. Try --help to see all options.")]
    NoArguments,
    #[error("Component {0} does't exist")]
    ComponentNotExists(String),
    #[error("File {0} does't exist")]
    FileNotExists(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Syntax error: {0}")]
    ReaderError(reader::error::E),
    #[error("Context error: {0}")]
    ContextError(context::E),
    #[error("Scenario error: {0}")]
    ScenarioError(scenario::E),
    #[error("Operator error: {0}")]
    OperatorError(operator::E),
    #[error("Error: {0}")]
    Other(String),
}

impl From<String> for E {
    fn from(e: String) -> Self {
        E::Other(e)
    }
}

impl From<reader::error::E> for E {
    fn from(e: reader::error::E) -> Self {
        E::ReaderError(e)
    }
}
impl From<LinkedErr<reader::error::E>> for E {
    fn from(e: LinkedErr<reader::error::E>) -> Self {
        E::ReaderError(e.e)
    }
}
impl From<error::E> for E {
    fn from(e: error::E) -> Self {
        E::Other(e.msg.to_owned())
    }
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        E::ContextError(e)
    }
}

impl From<scenario::E> for E {
    fn from(e: scenario::E) -> Self {
        E::ScenarioError(e)
    }
}

impl From<operator::E> for E {
    fn from(e: operator::E) -> Self {
        E::OperatorError(e)
    }
}

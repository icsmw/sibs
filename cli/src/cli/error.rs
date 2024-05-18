use crate::{
    error,
    error::LinkedErr,
    inf::{context, operator, scenario, value},
    reader,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Missed path to target file. Usage: {0} path_to_file")]
    NoPathToTargetFile(String),
    #[error("No any options/commands. Try --help to see all options.")]
    NoArguments,
    #[error("Next arguments cannot be used together: {0}")]
    NotSupportedMultipleArguments(String),
    #[error("Key {0} is defined multiple times")]
    DuplicateOfKey(String),
    #[error("Component {0} does't exist")]
    ComponentNotExists(String),
    #[error("After \"{0}\" argument is required")]
    NeedsArgumentAfter(String),
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
    #[error("Fail to execute.\n{0}")]
    OperatorError(operator::E),
    #[error("AnyValue error: {0}")]
    AnyValue(value::E),
    #[error("Error: {0}")]
    Other(String),
}

impl From<String> for E {
    fn from(e: String) -> Self {
        E::Other(e)
    }
}
impl From<value::E> for E {
    fn from(e: value::E) -> Self {
        E::AnyValue(e)
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
impl From<LinkedErr<operator::E>> for E {
    fn from(e: LinkedErr<operator::E>) -> Self {
        E::OperatorError(e.e)
    }
}

use crate::{
    error,
    inf::{context, operator, scenario},
    reader,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("--scenario requires a path to *.sibs file")]
    NoPathToScenarioFile,
    #[error("--help (-h) can be used in global scope or in component context. Try --help to see all options.")]
    InvalidHelpRequest,
    #[error("No any options/commands. Try --help to see all options.")]
    NoArguments,
    #[error("Terminal command has been finished with errors")]
    SpawningCommand,
    #[error("Task {0} doesn't have an actions block")]
    NoTaskBlock(String),
    #[error("No any task provided for component {0}")]
    NoTaskForComponent(String),
    #[error("Component {0} does't exist")]
    ComponentNotExists(String),
    #[error("Component {0} does't include task {1}")]
    TaskNotExists(String, String),
    #[error("File {0} does't exist")]
    FileNotExists(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Syntax error: {0}")]
    Reader(#[from] reader::error::E),
    #[error("CWD is required")]
    NoCurrentWorkingFolder,
    #[error("Not supported")]
    NotSupported,
    #[error("{0}: {1}")]
    FunctionError(String, String),
    #[error("Variable {0} isn't assigned")]
    VariableIsNotAssigned(String),
    #[error("Fail to extract value")]
    FailToExtractValue,
    #[error("Context error: {0}")]
    ContextError(String),
    #[error("Scenario error: {0}")]
    ScenarioError(String),
    #[error("Operator error: {0}")]
    OperatorError(String),
    #[error("Error: {0}")]
    Other(String),
}

impl From<error::E> for E {
    fn from(e: error::E) -> Self {
        E::Other(e.msg.to_owned())
    }
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        E::ContextError(e.to_string())
    }
}

impl From<scenario::E> for E {
    fn from(e: scenario::E) -> Self {
        E::ScenarioError(e.to_string())
    }
}

impl From<operator::E> for E {
    fn from(e: operator::E) -> Self {
        E::OperatorError(e.to_string())
    }
}

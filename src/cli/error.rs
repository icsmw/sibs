use crate::reader;
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
}

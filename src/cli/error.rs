use crate::reader;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum E {
    #[error("--scenario requires a path to *.sibs file")]
    NoPathToScenarioFile,
    #[error("--help (-h) can be used in global scope or in component context. Try --help to see all options.")]
    InvalidHelpRequest,

    #[error("File {0} does't exist")]
    FileNotExists(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Syntax error: {0}")]
    Reader(#[from] reader::error::E),
}

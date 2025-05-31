use diagnostics::LinkedErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Path doesn't exist: {0}")]
    PathDoesNotExist(String),
    #[error("Cannot find scenario file in current location: {0}")]
    ScenarioNotFound(String),
    #[error("IO error: {0}")]
    IO(String),
    #[error("--scenario requires a path to .sibs file")]
    MissedPathWithScenario,
    #[error("No arguments to get task's name")]
    FailToGetTaskName,
    #[error("No arguments to get component's name")]
    FailToGetComponentName,
    #[error(
        "Parameter \"{0}\" can be used as standalone only. Please use --help to get more details"
    )]
    StandaloneParameter(String),
    #[error("To run task component name should be provided")]
    NoComponentParameter,
    #[error("Fail to get cwd from \"{0}\"")]
    FailToGetCwd(String),
    #[error("Component \"{0}\" doesn't exists")]
    ComponentNotFound(String),
    #[error("LTS should be run without addition arguments")]
    SelfishLts,

    #[error("Fail to read valid scenario from \"{0}\"")]
    FailExtractAnchorNodeFrom(String),
    #[error("Script has been executed already")]
    ScriptAlreadyExecuted,
    #[error("Parser error: {0}")]
    Parser(parser::ParserError),
    #[error("Semantic error: {0}")]
    Semantic(semantic::SemanticError),
    #[error("Runtime error: {0}")]
    Runtime(runtime::RtError),
    #[error("Scenario error: {0}")]
    Scenario(scenario::ScenarioError),
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
    }
}

impl From<scenario::ScenarioError> for E {
    fn from(err: scenario::ScenarioError) -> Self {
        E::Scenario(err)
    }
}

impl From<parser::ParserError> for E {
    fn from(err: parser::ParserError) -> Self {
        E::Parser(err)
    }
}

impl From<LinkedErr<parser::ParserError>> for E {
    fn from(err: LinkedErr<parser::ParserError>) -> Self {
        E::Parser(err.e)
    }
}

impl From<semantic::SemanticError> for E {
    fn from(err: semantic::SemanticError) -> Self {
        E::Semantic(err)
    }
}

impl From<LinkedErr<semantic::SemanticError>> for E {
    fn from(err: LinkedErr<semantic::SemanticError>) -> Self {
        E::Semantic(err.e)
    }
}

impl From<runtime::RtError> for E {
    fn from(err: runtime::RtError) -> Self {
        E::Runtime(err)
    }
}

impl From<LinkedErr<runtime::RtError>> for E {
    fn from(err: LinkedErr<runtime::RtError>) -> Self {
        E::Runtime(err.e)
    }
}

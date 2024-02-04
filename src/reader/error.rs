use crate::{error, executors, inf::context};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Unknown variable type: {0}")]
    UnknownVariableType(String),
    #[error("Not closed variable type declaration")]
    NotClosedTypeDeclaration,
    #[error("No variable type declaration")]
    NoTypeDeclaration,
    #[error("Fail to find String ending")]
    NoStringEnd,
    #[error("Fail to find reference variable to string")]
    NoVariableReference,
    #[error("Fail to find closing \"}}\" injection into string")]
    NoInjectionClose,
    #[error("Not ascii variable value: {0}")]
    NotAsciiValue(String),
    #[error("Empty value")]
    EmptyValue,
    #[error("Using reserved chars")]
    UsingReservedChars,
    #[error("Missed semicolon")]
    MissedSemicolon,
    #[error("Invalid task arguments")]
    InvalidTaskArguments,
    #[error("No task arguments")]
    NoTaskArguments,
    #[error("Invalid task name")]
    InvalidTaskName,
    #[error("Fail find task actions, missed ]")]
    FailFindTaskActions,
    #[error("Nested functions arn't supported")]
    NestedFunction,
    #[error("No function on optional action")]
    NoFunctionOnOptionalAction,
    #[error("Nested optional action")]
    NestedOptionalAction,
    #[error("Fail parse optional action")]
    FailParseOptionalAction,
    #[error("Empty group")]
    EmptyGroup,
    #[error("No component name")]
    EmptyComponentName,
    #[error("Command has empty value")]
    EmptyCommand,
    #[error("Invalid component name")]
    InvalidComponentName,
    #[error("Fail get last token")]
    FailGetToken,
    #[error("Invalid variable name")]
    InvalidVariableName,
    #[error("Invalid block [...]")]
    InvalidBlock,
    #[error("No value after comparing ==")]
    NoValueAfterComparing,
    #[error("Empty path to reference")]
    EmptyPathToReference,
    #[error("Invalid reference")]
    InvalidReference,
    #[error("No destination function after >")]
    NoDestFunction,
    #[error("Missed comparing operator: == or !=")]
    MissedComparingOperator,
    #[error("Fail to find proviso of condition")]
    NoProvisoOfCondition,
    #[error("No loop variable EACH($var)")]
    NoLoopVariable,
    #[error("After AND or OR should be proviso")]
    RepeatedCombinationOperator,
    #[error("Only string values can be used with conditions")]
    NoStringValueWithCondition,
    #[error("Expecting = or ==")]
    NoComparingOrAssignation,
    #[error("No component body")]
    NoComponentBody,
    #[error("Group [...] is expecting")]
    NoGroup,
    #[error("Expecting whitespace after condition like OR, AND")]
    NoWhitespaceAfterCondition,
    #[error("No loop input EACH($var) input [...]")]
    NoLoopInput,
    #[error("Optional action doesn't have action")]
    NotActionForCondition,
    #[error("Not closed group")]
    NotClosedGroup,
    #[error("Not closed condition group (...)")]
    NotClosedConditionGroup,
    #[error("Nested condition group (..(..)..) aren't supported")]
    NestedConditionGroups,
    #[error("No values related to variable")]
    NoVariableValues,
    #[error("No metadata content")]
    NoMetaContent,
    #[error("Not allowed function")]
    NotAllowedFunction,
    #[error("Invalid function name")]
    InvalidFunctionName,
    #[error("Function isn't registred")]
    FunctionIsNotRegistred,
    #[error("Invalid function return")]
    InvalidFunctionReturn,
    #[error("Converting error")]
    Infallible(#[from] std::convert::Infallible),
    #[error("File {0} does't exist")]
    FileNotExists(String),
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("{0}: {1}")]
    OwnedError(String, String),
    #[error("Context error: {0}")]
    ContextError(String),
    #[error("Executor error: {0}")]
    ExecutorError(String),
}

impl From<error::E> for E {
    fn from(e: error::E) -> Self {
        E::OwnedError(e.sig, e.msg)
    }
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        E::ContextError(e.to_string())
    }
}

impl From<executors::E> for E {
    fn from(e: executors::E) -> Self {
        E::ExecutorError(e.to_string())
    }
}

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
    #[error("Missed semicolon")]
    MissedSemicolon,
    #[error("No task arguments")]
    NoTaskArguments,
    #[error("Fail find task actions, missed ]")]
    FailFindTaskActions,
    #[error("Nested functions arn't supported")]
    NestedFunction,
    #[error("No function on optional action")]
    NoFunctionOnOptionalAction,
    #[error("Fail parse optional action")]
    FailParseOptionalAction,
    #[error("Empty group")]
    EmptyGroup,
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
    // #[error("Function already redirected")]
    // FunctionAlreadyRedirected,
    #[error("Path doesn't include parent")]
    NoFileParent,
    #[error("Converting error")]
    Infallible(#[from] std::convert::Infallible),
    #[error("File {0} does't exist")]
    FileNotExists(String),
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("{0}: {1}")]
    FunctionError(String, String),
    #[error("{0}")]
    Other(String),
}

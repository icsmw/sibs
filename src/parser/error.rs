use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Found not ascii char: {0}")]
    NotAscii(char),
    #[error("Unexpected whitespace: {0}")]
    UnexpectedWhitespace(usize),
    #[error("Unexpected char: {0}")]
    UnexpectedChar(char),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
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
    #[error("No task actions")]
    NoTaskActions,
    #[error("Fail find task actions, missed ]")]
    FailFindTaskActions,
    #[error("Empty group")]
    EmptyGroup,
    #[error("Missed comparing operator: == or !=")]
    MissedComparingOperator,
    #[error("Fail to find proviso of condition")]
    NoProvisoOfCondition,
    #[error("After AND or OR should be proviso")]
    RepeatedCombinationOperator,
    #[error("Only string values can be used with conditions")]
    NoStringValueWithCondition,
    #[error("Expecting = or ==")]
    NoComparingOrAssignation,
    #[error("Unnamed component")]
    UnnamedComponent,
    #[error("No component context")]
    NoComponentContext,
    #[error("Group [...] is expecting")]
    NoGroup,
    #[error("Not closed group")]
    NotClosedGroup,
    #[error("Not closed condition group (...)")]
    NotClosedConditionGroup,
    #[error("Nested condition group (..(..)..) aren't supported")]
    NestedConditionGroups,
    #[error("No values related to variable")]
    NoVariableValues,
    #[error("Converting error")]
    Infallible(#[from] std::convert::Infallible),
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

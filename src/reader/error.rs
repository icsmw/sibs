use crate::{
    error,
    error::LinkedErr,
    executors,
    inf::{context, operator},
    reader::Reader,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Out of string bounds")]
    OutOfStringBounds,
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
    #[error("Fail to find injection into string")]
    FailToFindInjection,
    #[error("Fail to parse right side of assignation")]
    FailToParseRightSideOfAssignation,
    #[error("Not ascii variable value: {0}")]
    NotAsciiValue(String),
    #[error("Empty value")]
    EmptyValue,
    #[error("Error parsing an integer from string: {0}")]
    IntegerParsingError(String),
    #[error("No content before semicolon")]
    NoContentBeforeSemicolon,
    #[error("No function arguments")]
    NoFunctionArguments,
    #[error("Redundant comma")]
    RedundantComma,
    #[error("Using reserved chars")]
    UsingReservedChars,
    #[error("Missed semicolon")]
    MissedSemicolon,
    #[error("Unexpected semicolon")]
    UnexpectedSemicolon,
    #[error("\"{0}\" cannot parse task arguments")]
    InvalidTaskArguments(String),
    #[error("No task arguments: cannot parse task arguments; probably missed \")\"")]
    NoTaskArguments,
    #[error("\"{0}\" is invalid name of task")]
    InvalidTaskName(String),
    #[error("Fail find task actions, probably missed \"]\"")]
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
    #[error("Empty block")]
    EmptyBlock,
    #[error("Subsequence doesn't return value")]
    NoValueFromSubsequence,
    #[error("Subsequence's element doesn't return value")]
    NoValueFromSubsequenceElement,
    #[error("Fail to parse subsequence's element value")]
    FailToParseValueOfSubsequenceElement,
    #[error("Subsequence doesn't return bool value")]
    NoBoolValueFromSubsequence,
    #[error("Token {0} not found")]
    TokenNotFound(usize),
    #[error("Token {0} has invalid range; string len={1}; range [{2},{3}]")]
    TokenHasInvalidRange(usize, usize, usize, usize),
    #[error("No component name")]
    EmptyComponentName,
    #[error("Fail to read conditions")]
    FailToReadConditions,
    #[error("No command value")]
    EmptyCommand,
    #[error("This type of argument cannot be used in references")]
    InvalidArgumentForReference,
    #[error("No arguments are defined for reference")]
    NoArgumentsForReference,
    #[error("Function @import has invalid arguments")]
    ImportFunctionInvalidArgs,
    #[error("Function @import doesn't have arguments")]
    ImportFunctionNoArgs,
    #[error("Function @import expects a string as argument")]
    ImportFunctionInvalidPathArg,
    #[error("Fail to recognize code: \"{0}\"")]
    UnrecognizedCode(String),
    #[error("\"{0}\" is an invalid component name")]
    InvalidComponentName(String),
    #[error("Fail get last token")]
    FailGetToken,
    #[error("Invalid variable name")]
    InvalidVariableName,
    #[error("No value after comparing ==")]
    NoValueAfterComparing,
    #[error("Empty path to reference")]
    EmptyPathToReference,
    #[error("\"{0}\" is an invalid reference")]
    InvalidReference(String),
    #[error("No destination function after >")]
    NoDestFunction,
    #[error("Fail to find proviso of condition")]
    NoProvisoOfCondition,
    #[error("No loop variable EACH($var)")]
    NoLoopVariable,
    #[error("No loop variable declaration; expecting: EACH($var)")]
    NoLoopInitialization,
    #[error("After AND or OR should be proviso")]
    RepeatedCombinationOperator,
    #[error("Expecting = or ==")]
    NoComparingOrAssignation,
    #[error("Fail to parse one of sides of IF comparing")]
    FailToParseSideOfComparing,
    #[error("Not supported input for IF statement; supported: string, variable, function")]
    NotSupportedInputForIF,
    #[error("No component body")]
    NoComponentBody,
    #[error("No FIRST statement body")]
    NoFIRSTStatementBody,
    #[error("Group [...] is expecting")]
    NoGroup,
    #[error("No component definition #(...)")]
    NoComponentDefinition,
    #[error("Fail to find optional redirection: \"=>\"")]
    NoOptionalRedirection,
    #[error("Invalid action is used for optional statements")]
    InvalidActionForOptional,
    #[error("Fail to detect an action for optional statements")]
    FailFindActionForOptional,
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
    #[error("Fail to find condition(s) for IF statement")]
    NoConditionForIfStatement,
    #[error("Fail to find actions block for IF statement")]
    NoBlockForIfStatement,
    #[error("Main actions block for IF statement is missed")]
    NoMainBlockForIfStatement,
    #[error("No values related to variable")]
    NoVariableValues,
    #[error("No metadata content")]
    NoMetaContent,
    #[error("Not allowed function")]
    NotAllowedFunction,
    #[error("Invalid function name: {0}")]
    InvalidFunctionName(String),
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
    ContextError(context::E),
    #[error("Executor error: {0}")]
    ExecutorError(executors::E),
    #[error("Operator error: {0}")]
    OperatorError(String),
}

impl E {
    pub fn linked(self, token: &usize) -> LinkedErr<E> {
        LinkedErr {
            token: Some(*token),
            e: self,
        }
    }
    pub fn by_reader(self, reader: &Reader) -> LinkedErr<E> {
        match reader.token() {
            Ok(token) => self.linked(&token.id),
            Err(e) => e.unlinked(),
        }
    }
    pub fn unlinked(self) -> LinkedErr<E> {
        LinkedErr {
            token: None,
            e: self,
        }
    }
}

impl From<E> for LinkedErr<E> {
    fn from(e: E) -> Self {
        e.unlinked()
    }
}
impl From<operator::E> for LinkedErr<E> {
    fn from(e: operator::E) -> Self {
        E::OperatorError(e.to_string()).unlinked()
    }
}

impl From<context::E> for LinkedErr<E> {
    fn from(e: context::E) -> Self {
        E::ContextError(e).unlinked()
    }
}

impl From<executors::E> for LinkedErr<E> {
    fn from(e: executors::E) -> Self {
        E::ExecutorError(e).unlinked()
    }
}

impl From<std::io::Error> for LinkedErr<E> {
    fn from(e: std::io::Error) -> Self {
        E::IO(e).unlinked()
    }
}

impl From<error::E> for E {
    fn from(e: error::E) -> Self {
        E::OwnedError(e.sig, e.msg)
    }
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        E::ContextError(e)
    }
}

impl From<executors::E> for E {
    fn from(e: executors::E) -> Self {
        E::ExecutorError(e)
    }
}

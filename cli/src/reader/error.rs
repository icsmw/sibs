use std::path::PathBuf;

use crate::{
    error,
    error::LinkedErr,
    functions,
    inf::{context, journal, map, operator},
    reader::Reader,
};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Fail to detect parent folder for: {0}")]
    NoCurrentWorkingFolder(PathBuf),
    #[error("Fail to find a token {0}")]
    FailToFindToken(usize),
    #[error("Unknown variable type: {0}")]
    UnknownVariableType(String),
    #[error("Not closed variable type declaration")]
    NotClosedTypeDeclaration,
    #[error("Not closed function arguments (..)")]
    NotClosedFunctionArgs,
    #[error("No variable type declaration")]
    NoTypeDeclaration,
    #[error("Fail to find String ending")]
    NoStringEnd,
    #[error("Fail to find closing \"}}\" injection into string")]
    NoInjectionClose,
    #[error("Fail to find injection into string")]
    FailToFindInjection,
    #[error("Fail to parse right side of assignation")]
    FailToParseRightSideOfAssignation,
    #[error("Invalid usage \"global\" keyword")]
    InvalidUsageGlobalKeyword,
    #[error("Not ascii variable value: {0}")]
    NotAsciiValue(String),
    #[error("Empty value")]
    EmptyValue,
    #[error("Error parsing an integer from string: {0}")]
    IntegerParsingError(String),
    #[error("No content before semicolon")]
    NoContentBeforeSemicolon,
    #[error("Using reserved chars")]
    UsingReservedChars,
    #[error("Missed semicolon")]
    MissedSemicolon,
    #[error("Only import function can be used in the root scope")]
    OnlyImportFunctionAllowedOnRoot,
    #[error("\"{0}\" cannot parse task arguments")]
    InvalidTaskArguments(String),
    #[error("No task arguments: cannot parse task arguments; probably missed \")\"")]
    NoTaskArguments,
    #[error("\"{0}\" is invalid name of task")]
    InvalidTaskName(String),
    #[error("Fail find task actions, probably missed \"]\"")]
    FailFindTaskActions,
    #[error("Empty block")]
    EmptyBlock,
    #[error("File {0} already has a map")]
    FileAlreadyHasMap(PathBuf),
    #[error("Subsequence doesn't return value")]
    NoValueFromSubsequence,
    #[error("Subsequence's element doesn't return value")]
    NoValueFromSubsequenceElement,
    #[error("Fail to parse subsequence's element value")]
    FailToParseValueOfSubsequenceElement,
    #[error("Subsequence doesn't return bool value")]
    NoBoolValueFromSubsequence,
    #[error("No component name")]
    EmptyComponentName,
    #[error("Fail to read conditions")]
    FailToReadConditions,
    #[error("This type of argument cannot be used in references")]
    InvalidArgumentForReference,
    #[error("Function import has invalid arguments")]
    ImportFunctionInvalidArgs,
    #[error("Fail to recognize code: \"{0}\"")]
    UnrecognizedCode(String),
    #[error("\"{0}\" is an invalid component name")]
    InvalidComponentName(String),
    #[error("Fail get last token")]
    FailGetToken,
    #[error("Invalid variable name")]
    InvalidVariableName,
    #[error("Empty path to reference")]
    EmptyPathToReference,
    #[error("\"{0}\" is an invalid reference")]
    InvalidReference(String),
    #[error("No destination function after >")]
    NoDestFunction,
    #[error("No loop variable each($var)")]
    NoLoopVariable,
    #[error("No loop variable declaration; expecting: each($var)")]
    NoLoopInitialization,
    #[error("No component body")]
    NoComponentBody,
    #[error("No first statement body")]
    NoFIRSTStatementBody,
    #[error("No join statement body; join(<ref; ref; ...>)")]
    NoJOINStatementBody,
    #[error("join statement can include only references to tasks; join(<ref; ref; ...>)")]
    NotReferenceInJOIN,
    #[error("Group [...] is expecting")]
    NoGroup,
    #[error("No component definition #(...)")]
    NoComponentDefinition,
    #[error("Fail to find optional redirection: \"=>\"")]
    NoOptionalRedirection,
    #[error("Fail to find related task for gatekeeper: \"->\"")]
    NoReferenceForGatekeeper,
    #[error("Gatekeeper should be refered to task")]
    GatekeeperShouldRefToTask,
    #[error("Fail to detect an action for optional statements")]
    FailFindActionForOptional,
    #[error("No loop input each($var) input [...]")]
    NoLoopInput,
    #[error("Fail to find condition(s) for if statement")]
    NoConditionForIfStatement,
    #[error("Fail to find actions block for if statement")]
    NoBlockForIfStatement,
    #[error("Main actions block for if statement is missed")]
    NoMainBlockForIfStatement,
    #[error("No values related to variable")]
    NoVariableValues,
    #[error("No metadata content")]
    NoMetaContent,
    #[error("Invalid function name: {0}")]
    InvalidFunctionName(String),
    #[error("Converting error")]
    Infallible(#[from] std::convert::Infallible),
    #[error("IO error")]
    IO(String),
    #[error("{0}: {1}")]
    OwnedError(String, String),
    #[error("{0}")]
    ContextError(context::E),
    #[error("{0}")]
    ExecutorError(functions::E),
    #[error("{0}")]
    OperatorError(Box<operator::E>),
    #[error("{0}")]
    MapError(map::E),
    #[error("{0}")]
    JournalError(journal::E),
}

impl E {
    pub fn linked(self, token: &usize) -> LinkedErr<E> {
        LinkedErr::new(self, Some(*token))
    }
    pub fn by_reader(self, reader: &Reader) -> LinkedErr<E> {
        match reader.token() {
            Ok(token) => self.linked(&token.id),
            Err(e) => e.unlinked(),
        }
    }
    pub fn unlinked(self) -> LinkedErr<E> {
        LinkedErr::new(self, None)
    }
}

impl From<E> for LinkedErr<E> {
    fn from(e: E) -> Self {
        e.unlinked()
    }
}
impl From<map::E> for E {
    fn from(value: map::E) -> Self {
        E::MapError(value)
    }
}

impl From<journal::E> for LinkedErr<E> {
    fn from(e: journal::E) -> Self {
        E::JournalError(e).unlinked()
    }
}

impl From<operator::E> for LinkedErr<E> {
    fn from(e: operator::E) -> Self {
        E::OperatorError(Box::new(e)).unlinked()
    }
}

impl From<context::E> for LinkedErr<E> {
    fn from(e: context::E) -> Self {
        E::ContextError(e).unlinked()
    }
}

impl From<functions::E> for LinkedErr<E> {
    fn from(e: functions::E) -> Self {
        E::ExecutorError(e).unlinked()
    }
}

impl From<std::io::Error> for LinkedErr<E> {
    fn from(e: std::io::Error) -> Self {
        E::IO(e.to_string()).unlinked()
    }
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
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

impl From<functions::E> for E {
    fn from(e: functions::E) -> Self {
        E::ExecutorError(e)
    }
}

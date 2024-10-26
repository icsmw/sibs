use thiserror::Error;

use crate::lexer::*;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Token {0} doesn't have constant length")]
    NoConstantLength(KindId),
    #[error("Token {0} cannot be converted to origin token")]
    CannotConvertToKind(KindId),
    #[error("Invalid number")]
    InvalidNumber,
    #[error("Attempt to read EOF or BOF")]
    AttemptToReadEOForBOF,
    #[error("{0:?} cannot be converted to char")]
    CannotConverToChar(KindId),
    #[error("Cannot find closing symbol: {0}")]
    NoClosingSymbol(char),
    #[error("Cannot read a group between {0} and {1}")]
    CannotReadGroupBetween(char, char),
    #[error("Cannot recognize content from position: {0}")]
    FailRecognizeContent(usize),
    #[error("Next tokens are in conflict: {0}")]
    TokensAreInConflict(String),
}

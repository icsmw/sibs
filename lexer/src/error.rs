use thiserror::Error;

use crate::*;

/// Represents the possible errors that can occur during lexing.
///
/// This enum defines various error types that may be encountered
/// while processing tokens in the lexer.
#[derive(Error, Debug, Clone)]
pub enum E {
    /// Error indicating a token does not have a constant length.
    #[error("Token {0} doesn't have constant length")]
    NoConstantLength(KindId),

    /// Error indicating a `KindId` cannot be converted to its corresponding `Kind`.
    #[error("Token {0} cannot be converted to origin token")]
    CannotConvertToKind(KindId),

    /// Error indicating an invalid number was encountered.
    #[error("Invalid number")]
    InvalidNumber,

    /// Error indicating an attempt to read the end or beginning of the file.
    #[error("Attempt to read EOF or BOF")]
    AttemptToReadEOForBOF,

    /// Error indicating a `KindId` cannot be converted to a character.
    #[error("{0:?} cannot be converted to char")]
    CannotConverToChar(KindId),

    /// Error indicating a closing symbol was not found.
    #[error("Cannot find closing symbol: {0}")]
    NoClosingSymbol(char),

    /// Error indicating a group cannot be read between two symbols.
    #[error("Cannot read a group between {0} and {1}")]
    CannotReadGroupBetween(char, char),

    /// Error indicating failure to recognize content from a certain position.
    #[error("Cannot recognize content from position: {0}")]
    FailRecognizeContent(usize),

    /// Error indicating that the next tokens are in conflict.
    #[error("Next tokens are in conflict: {0}")]
    TokensAreInConflict(String),
}

use crate::*;

/// Trait for calculating the length of tokens with a constant size.
///
/// This trait is implemented for token kinds that have a fixed length representation.
/// For tokens like identifiers, numbers, or strings, which can vary in length,
/// the `length` method will return an error.
pub(crate) trait ConstantLength {
    /// Returns the constant length of the token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token does not have a constant length.
    fn length(&self) -> Result<usize, E>;
}

impl ConstantLength for KindId {
    fn length(&self) -> Result<usize, E> {
        match self {
            Self::EOF | Self::BOF => Ok(0),
            Self::SingleQuote
            | Self::DoubleQuote
            | Self::Backslash
            | Self::Tilde
            | Self::Backtick
            | Self::Question
            | Self::Dollar
            | Self::At
            | Self::Pound
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equals
            | Self::Less
            | Self::Greater
            | Self::VerticalBar
            | Self::Bang
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Comma
            | Self::Colon
            | Self::Dot
            | Self::Semicolon
            | Self::CR
            | Self::LF => Ok(1),
            Self::EqualEqual
            | Self::BangEqual
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::DotDot
            | Self::Arrow
            | Self::DoubleArrow
            | Self::Comment
            | Self::CRLF => Ok(2),
            Self::Meta => Ok(3),
            Self::Identifier
            | Self::Number
            | Self::String
            | Self::Literal
            | Self::Whitespace
            | Self::Keyword => Err(E::NoConstantLength(self.clone())),
        }
    }
}

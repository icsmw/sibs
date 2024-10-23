use crate::lexer::*;

pub trait ConstantLength {
    /// Returns length of only any content. Identifer as valid alphanumeric should not
    /// return a length
    fn length(&self) -> Result<usize, E>;
}

impl ConstantLength for KindId {
    fn length(&self) -> Result<usize, E> {
        match self {
            Self::Question
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
            Self::If
            | Self::EqualEqual
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
            Self::Meta | Self::For | Self::Let => Ok(3),
            Self::Else | Self::Loop | Self::True => Ok(4),
            Self::While | Self::False => Ok(5),
            Self::Return => Ok(6),
            Self::Identifier
            | Self::Number
            | Self::String
            | Self::InterpolatedString
            | Self::Command
            | Self::EOF
            | Self::BOF => Err(E::NoConstantLength(self.clone())),
        }
    }
}

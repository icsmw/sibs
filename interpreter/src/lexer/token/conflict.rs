use crate::lexer::*;

pub trait ConflictResolver {
    /// Returns length of only any content. Identifer as valid alphanumeric should not
    /// return a length
    fn resolve_conflict(&self, id: &KindId) -> KindId;
}

impl ConflictResolver for KindId {
    fn resolve_conflict(&self, id: &KindId) -> KindId {
        match self {
            Self::If
            | Self::Else
            | Self::While
            | Self::Loop
            | Self::For
            | Self::Return
            | Self::Let
            | Self::True
            | Self::False
            | Self::Identifier
            | Self::Number
            | Self::String
            | Self::InterpolatedString
            | Self::Command
            | Self::EOF
            | Self::BOF
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
            | Self::EqualEqual
            | Self::BangEqual
            | Self::Less
            | Self::LessEqual
            | Self::Greater
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::VerticalBar
            | Self::Bang
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Comma
            | Self::Colon
            | Self::Dot
            | Self::DotDot
            | Self::Semicolon
            | Self::Arrow
            | Self::DoubleArrow
            | Self::LF
            | Self::CR
            | Self::CRLF => self.clone(),
            Self::Comment => {
                if matches!(id, KindId::Meta) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
            Self::Meta => {
                if matches!(id, KindId::Comment) {
                    self.clone()
                } else {
                    id.clone()
                }
            }
        }
    }
}

use crate::lexer::*;

pub trait ConflictResolver {
    /// Returns length of only any content. Identifer as valid alphanumeric should not
    /// return a length
    fn resolve_conflict(&self, id: &KindId) -> KindId;
}

impl ConflictResolver for KindId {
    fn resolve_conflict(&self, id: &KindId) -> KindId {
        match self {
            Self::Else
            | Self::While
            | Self::Loop
            | Self::For
            | Self::Return
            | Self::Let
            | Self::True
            | Self::False
            | Self::String
            | Self::InterpolatedString
            | Self::Command
            | Self::EOF
            | Self::BOF
            | Self::Question
            | Self::Dollar
            | Self::At
            | Self::Pound
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::BangEqual
            | Self::Greater
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::VerticalBar
            | Self::Bang
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
            | Self::CRLF
            | Self::SingleQuote
            | Self::DoubleQuote
            | Self::Tilde
            | Self::Backtick
            | Self::Whitespace => self.clone(),
            Self::Plus => {
                if matches!(id, KindId::PlusEqual) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
            Self::PlusEqual => {
                if matches!(id, KindId::Plus) {
                    self.clone()
                } else {
                    id.clone()
                }
            }
            Self::Less => {
                if matches!(id, KindId::LessEqual) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
            Self::LessEqual => {
                if matches!(id, KindId::Less) {
                    self.clone()
                } else {
                    id.clone()
                }
            }
            Self::Minus => {
                if matches!(id, KindId::MinusEqual) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
            Self::MinusEqual => {
                if matches!(id, KindId::Minus) {
                    self.clone()
                } else {
                    id.clone()
                }
            }
            Self::Number => {
                if matches!(id, KindId::Identifier) {
                    self.clone()
                } else {
                    id.clone()
                }
            }
            Self::Equals => {
                if matches!(id, KindId::EqualEqual) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
            Self::EqualEqual => {
                if matches!(id, KindId::Equals) {
                    self.clone()
                } else {
                    id.clone()
                }
            }
            Self::If => {
                if matches!(id, KindId::Identifier) {
                    self.clone()
                } else {
                    id.clone()
                }
            }
            Self::Identifier => {
                if matches!(
                    id,
                    KindId::If
                        | KindId::Else
                        | KindId::While
                        | KindId::Loop
                        | KindId::For
                        | KindId::Return
                        | KindId::Let
                        | KindId::True
                        | KindId::False
                        | KindId::Number
                ) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
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

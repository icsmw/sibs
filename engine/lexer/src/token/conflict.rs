use crate::*;

/// Trait for resolving conflicts between tokens of type `KindId`.
///
/// This trait defines a method for determining the correct `KindId` when there is
/// a potential conflict between two token kinds.
pub(crate) trait ConflictResolver {
    /// Resolves a conflict between `self` and another `KindId`.
    ///
    /// Returns the appropriate `KindId` based on conflict resolution rules.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to another `KindId` to resolve the conflict with.
    fn resolve_conflict(&self, id: &KindId) -> KindId;
}

impl ConflictResolver for KindId {
    fn resolve_conflict(&self, id: &KindId) -> KindId {
        match self {
            Self::Keyword
            | Self::String
            | Self::Literal
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
            | Self::Backslash
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
            Self::Identifier => match id {
                KindId::Keyword | KindId::Number => id.clone(),
                KindId::SingleQuote
                | KindId::DoubleQuote
                | KindId::Backslash
                | KindId::Tilde
                | KindId::Backtick
                | KindId::Question
                | KindId::Dollar
                | KindId::At
                | KindId::Pound
                | KindId::Plus
                | KindId::Minus
                | KindId::Star
                | KindId::Slash
                | KindId::Percent
                | KindId::Equals
                | KindId::EqualEqual
                | KindId::BangEqual
                | KindId::Less
                | KindId::LessEqual
                | KindId::Greater
                | KindId::GreaterEqual
                | KindId::And
                | KindId::Or
                | KindId::VerticalBar
                | KindId::Bang
                | KindId::PlusEqual
                | KindId::MinusEqual
                | KindId::StarEqual
                | KindId::SlashEqual
                | KindId::LeftParen
                | KindId::RightParen
                | KindId::LeftBrace
                | KindId::RightBrace
                | KindId::LeftBracket
                | KindId::RightBracket
                | KindId::Comma
                | KindId::Colon
                | KindId::Dot
                | KindId::DotDot
                | KindId::Semicolon
                | KindId::Arrow
                | KindId::DoubleArrow
                | KindId::LF
                | KindId::CR
                | KindId::CRLF
                | KindId::EOF
                | KindId::BOF
                | KindId::Identifier
                | KindId::String
                | KindId::Literal
                | KindId::Whitespace
                | KindId::Comment
                | KindId::Meta => self.clone(),
            },
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

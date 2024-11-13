use crate::*;
use lexer::{KindId, Token};

impl Interest for StatementId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Break => matches!(token.id(), KindId::Keyword),
            Self::Return => matches!(token.id(), KindId::Keyword),
            Self::For => matches!(token.id(), KindId::Keyword),
            Self::Loop => matches!(token.id(), KindId::Keyword),
            Self::While => matches!(token.id(), KindId::Keyword),
            Self::Each => matches!(token.id(), KindId::Keyword),
            Self::Assignation | Self::AssignedValue | Self::Join | Self::OneOf => {
                matches!(token.id(), KindId::Identifier)
            }
            Self::Optional => matches!(
                token.id(),
                KindId::LeftParen | KindId::Identifier | KindId::Number | KindId::Keyword
            ),
            Self::Block => matches!(token.id(), KindId::LeftBrace),
            Self::If => matches!(token.id(), KindId::Keyword),
        }
    }
}

use crate::*;
use lexer::{KindId, Token};

impl Interest for StatementId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::Break => matches!(token.id(), KindId::Break),
            Self::Return => matches!(token.id(), KindId::Return),
            Self::For => matches!(token.id(), KindId::For),
            Self::Loop => matches!(token.id(), KindId::Loop),
            Self::While => matches!(token.id(), KindId::While),
            Self::Each => matches!(token.id(), KindId::Each),
            Self::Assignation | Self::AssignedValue | Self::Join | Self::OneOf => {
                matches!(token.id(), KindId::Identifier)
            }
            Self::Optional => matches!(
                token.id(),
                KindId::LeftParen
                    | KindId::Identifier
                    | KindId::Number
                    | KindId::True
                    | KindId::False
            ),
            Self::Block => matches!(token.id(), KindId::LeftBrace),
            Self::If => matches!(token.id(), KindId::If),
        }
    }
}

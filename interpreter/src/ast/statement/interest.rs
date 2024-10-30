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
            Self::Each => matches!(token.id(), KindId::Identifier),
            Self::Assignation => matches!(token.id(), KindId::Identifier),
        }
    }
}

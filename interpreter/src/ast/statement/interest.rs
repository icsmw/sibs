use crate::*;
use lexer::{KindId, Token};

impl Interest for StatementId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::Break => matches!(token.id(), KindId::Break),
            Self::Return => matches!(token.id(), KindId::Return),
        }
    }
}

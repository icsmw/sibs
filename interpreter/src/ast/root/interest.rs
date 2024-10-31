use crate::*;
use lexer::{KindId, Token};

impl Interest for RootId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::Component => matches!(token.id(), KindId::Pound),
            Self::Task => matches!(token.id(), KindId::At),
        }
    }
}

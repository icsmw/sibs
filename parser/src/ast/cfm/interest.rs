use crate::*;
use asttree::*;
use lexer::{KindId, Token};

impl Interest for ControlFlowModifierId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Gatekeeper => matches!(token.id(), KindId::Pound),
            Self::Skip => matches!(token.id(), KindId::Identifier),
        }
    }
}

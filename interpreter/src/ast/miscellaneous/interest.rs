use crate::*;
use lexer::{KindId, Token};

impl Interest for MiscellaneousId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::Command => matches!(token.id(), KindId::Command),
        }
    }
}

use crate::*;
use lexer::{KindId, Token};

impl Interest for MiscellaneousId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Meta => matches!(token.id(), KindId::Meta),
            Self::Comment => matches!(token.id(), KindId::Comment),
        }
    }
}

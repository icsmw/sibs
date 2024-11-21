use crate::*;
use asttree::*;
use lexer::{Keyword, Kind, KindId, Token};

impl Interest for MiscellaneousId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Meta => matches!(token.id(), KindId::Meta),
            Self::Comment => matches!(token.id(), KindId::Comment),
            Self::Include => matches!(token.kind, Kind::Keyword(Keyword::Include)),
            Self::Module => matches!(token.kind, Kind::Keyword(Keyword::Mod)),
        }
    }
}

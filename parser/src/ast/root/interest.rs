use crate::*;

impl Interest for RootId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Component => matches!(token.id(), KindId::Pound),
            Self::Task => matches!(token.id(), KindId::At),
        }
    }
}

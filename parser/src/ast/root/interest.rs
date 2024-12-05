use crate::*;

impl Interest for RootId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Component => matches!(token.kind, Kind::Keyword(Keyword::Component)),
            Self::Task => matches!(
                token.kind,
                Kind::Keyword(Keyword::Task) | Kind::Keyword(Keyword::Private)
            ),
        }
    }
}

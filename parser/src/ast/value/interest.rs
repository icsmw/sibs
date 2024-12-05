use crate::*;

impl Interest for ValueId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::InterpolatedString => matches!(token.id(), KindId::InterpolatedString),
            Self::PrimitiveString => matches!(token.id(), KindId::String),
            Self::Number => matches!(token.id(), KindId::Number),
            Self::Boolean => matches!(
                token.kind,
                Kind::Keyword(Keyword::False) | Kind::Keyword(Keyword::True)
            ),
            Self::Array => matches!(token.id(), KindId::LeftBracket),
            Self::Error => matches!(token.kind, Kind::Keyword(Keyword::Error)),
        }
    }
}

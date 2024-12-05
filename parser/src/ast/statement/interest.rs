use crate::*;

impl Interest for StatementId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Join => matches!(token.kind, Kind::Keyword(Keyword::Join)),
            Self::OneOf => matches!(token.kind, Kind::Keyword(Keyword::OneOf)),
            Self::Break => matches!(token.kind, Kind::Keyword(Keyword::Break)),
            Self::Return => matches!(token.kind, Kind::Keyword(Keyword::Return)),
            Self::For => matches!(token.kind, Kind::Keyword(Keyword::For)),
            Self::Loop => matches!(token.kind, Kind::Keyword(Keyword::Loop)),
            Self::While => matches!(token.kind, Kind::Keyword(Keyword::While)),
            Self::Each => matches!(token.kind, Kind::Keyword(Keyword::Each)),
            Self::If => matches!(token.kind, Kind::Keyword(Keyword::If)),
            Self::Block => matches!(token.kind, Kind::LeftBrace),
            Self::Assignation => {
                matches!(token.id(), KindId::Identifier)
            }
            Self::AssignedValue => matches!(token.kind, Kind::Equals),
            // Same as for Expression::ComparisonSeq
            Self::Optional => matches!(
                token.kind,
                Kind::LeftParen
                    | Kind::Identifier(..)
                    | Kind::Number(..)
                    | Kind::String(..)
                    | Kind::InterpolatedString(..)
                    | Kind::Keyword(Keyword::True)
                    | Kind::Keyword(Keyword::False)
            ),
        }
    }
}

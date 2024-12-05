use crate::*;

impl Interest for ExpressionId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Command => matches!(token.id(), KindId::Command),
            Self::Call => matches!(token.kind, Kind::Dot),
            Self::Accessor => matches!(token.kind, Kind::LeftBracket),
            Self::Variable | Self::FunctionCall | Self::CompoundAssignments => {
                matches!(token.id(), KindId::Identifier)
            }
            Self::LogicalOp => matches!(token.kind, Kind::And | Kind::Or),
            Self::ComparisonOp => matches!(
                token.kind,
                Kind::Less
                    | Kind::LessEqual
                    | Kind::Greater
                    | Kind::GreaterEqual
                    | Kind::EqualEqual
                    | Kind::BangEqual
            ),
            Self::Comparison => matches!(
                token.kind,
                Kind::Identifier(..)
                    | Kind::Number(..)
                    | Kind::String(..)
                    | Kind::InterpolatedString(..)
                    | Kind::Keyword(Keyword::True)
                    | Kind::Keyword(Keyword::False)
            ),
            Self::ComparisonSeq => matches!(
                token.kind,
                Kind::LeftParen
                    | Kind::Identifier(..)
                    | Kind::Number(..)
                    | Kind::String(..)
                    | Kind::InterpolatedString(..)
                    | Kind::Keyword(Keyword::True)
                    | Kind::Keyword(Keyword::False)
            ),
            Self::ComparisonGroup | Self::BinaryExpGroup => matches!(token.kind, Kind::LeftParen),
            Self::Range => matches!(token.id(), KindId::Identifier | KindId::Number),
            Self::BinaryOp => {
                matches!(
                    token.kind,
                    Kind::Plus | Kind::Minus | Kind::Star | Kind::Slash
                )
            }
            Self::BinaryExp => matches!(token.id(), KindId::Identifier | KindId::Number),
            Self::BinaryExpSeq => matches!(
                token.id(),
                KindId::Identifier | KindId::Number | KindId::LeftParen
            ),
            Self::CompoundAssignmentsOp => {
                matches!(
                    token.kind,
                    Kind::PlusEqual | Kind::MinusEqual | Kind::StarEqual | Kind::SlashEqual
                )
            }
            Self::TaskCall => matches!(token.kind, Kind::Colon),
        }
    }
}

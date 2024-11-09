use crate::*;
use lexer::{KindId, Token};

impl Interest for ExpressionId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::Command => matches!(token.id(), KindId::Command),
            Self::Call => matches!(token.id(), KindId::Dot),
            Self::Accessor => matches!(token.id(), KindId::LeftBracket),
            Self::Variable | Self::FunctionCall | Self::CompoundAssignments => {
                matches!(token.id(), KindId::Identifier)
            }
            Self::LogicalOp => matches!(token.id(), KindId::And | KindId::Or),
            Self::ComparisonOp => matches!(
                token.id(),
                KindId::Less
                    | KindId::LessEqual
                    | KindId::Greater
                    | KindId::GreaterEqual
                    | KindId::EqualEqual
                    | KindId::BangEqual
            ),
            Self::Comparison => matches!(
                token.id(),
                KindId::Identifier | KindId::Number | KindId::True | KindId::False
            ),
            Self::ComparisonSeq => matches!(
                token.id(),
                KindId::LeftParen
                    | KindId::Identifier
                    | KindId::Number
                    | KindId::True
                    | KindId::False
            ),
            Self::ComparisonGroup | Self::BinaryExpGroup => matches!(token.id(), KindId::LeftParen),
            Self::Condition => matches!(
                token.id(),
                KindId::LeftParen
                    | KindId::Identifier
                    | KindId::Number
                    | KindId::True
                    | KindId::False
            ),
            Self::Range => matches!(token.id(), KindId::Identifier | KindId::Number),
            Self::CompoundAssignmentsOp => {
                matches!(
                    token.id(),
                    KindId::PlusEqual | KindId::MinusEqual | KindId::StarEqual | KindId::SlashEqual
                )
            }

            Self::BinaryExp | Self::BinaryExpSeq => matches!(
                token.id(),
                KindId::Identifier | KindId::Number | KindId::LeftParen
            ),
            Self::BinaryExpPri => matches!(token.id(), KindId::Identifier | KindId::Number),
            Self::BinaryOp => {
                matches!(
                    token.id(),
                    KindId::Plus | KindId::Minus | KindId::Star | KindId::Slash
                )
            }
            Self::TaskCall => matches!(token.id(), KindId::Colon),
        }
    }
}

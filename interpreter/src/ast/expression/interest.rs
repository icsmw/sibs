use crate::*;
use lexer::{KindId, Token};

impl Interest for ExpressionId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::Command => matches!(token.id(), KindId::Command),
            Self::Call => matches!(token.id(), KindId::Dot),
            Self::Accessor => matches!(token.id(), KindId::LeftBracket),
            Self::Variable | Self::FunctionCall | Self::Incrementer => {
                matches!(token.id(), KindId::Identifier)
            }
            Self::LogicalOp => matches!(token.id(), KindId::And | KindId::Or),
            Self::Comparing => matches!(
                token.id(),
                KindId::Identifier | KindId::Number | KindId::True | KindId::False
            ),
            Self::ComparingSeq => matches!(token.id(), KindId::LeftParen),
            Self::Condition => matches!(
                token.id(),
                KindId::LeftParen
                    | KindId::Identifier
                    | KindId::Number
                    | KindId::True
                    | KindId::False
            ),
            Self::Range => matches!(token.id(), KindId::Identifier | KindId::Number),
            Self::BinaryExp => matches!(token.id(), KindId::Identifier | KindId::Number),
            Self::TaskCall => matches!(token.id(), KindId::Colon),
        }
    }
}

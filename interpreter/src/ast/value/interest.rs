use crate::*;
use lexer::{KindId, Token};

impl Interest for ValueId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::InterpolatedString => matches!(token.id(), KindId::InterpolatedString),
            Self::PrimitiveString => matches!(token.id(), KindId::String),
            Self::Number => matches!(token.id(), KindId::Number),
            Self::Boolean => matches!(token.id(), KindId::True | KindId::False),
            Self::Array => matches!(token.id(), KindId::LeftBracket),
            Self::Error => matches!(token.id(), KindId::Identifier),
        }
    }
}

use crate::*;
use lexer::{KindId, Token};

impl Interest for ExpressionId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::Variable => matches!(token.id(), KindId::Identifier),
        }
    }
}

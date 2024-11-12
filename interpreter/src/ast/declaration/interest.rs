use crate::*;
use lexer::{KindId, Token};

impl Interest for DeclarationId {
    fn intrested(&self, token: &Token, _nodes: &Nodes) -> bool {
        match self {
            Self::VariableDeclaration | Self::VariableVariants => {
                matches!(token.id(), KindId::Identifier)
            }
            Self::VariableType => matches!(token.id(), KindId::Identifier),
            Self::VariableTypeDeclaration => matches!(token.id(), KindId::Colon),
            Self::Closure => matches!(token.id(), KindId::LeftParen),
        }
    }
}

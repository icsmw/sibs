use crate::*;
use lexer::{Keyword, Kind, Token};

impl Interest for DeclarationId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::VariableDeclaration | Self::VariableVariants => {
                matches!(token.kind, Kind::Identifier(..))
            }
            Self::VariableType => matches!(
                token.kind,
                Kind::Keyword(Keyword::Str)
                    | Kind::Keyword(Keyword::Bool)
                    | Kind::Keyword(Keyword::Num)
                    | Kind::Keyword(Keyword::Vec)
            ),
            Self::VariableTypeDeclaration => matches!(token.kind, Kind::Colon),
            Self::Closure => matches!(token.kind, Kind::LeftParen),
        }
    }
}

use crate::*;
use asttree::*;
use lexer::{Keyword, Kind, Token};

impl Interest for DeclarationId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::ArgumentDeclaration | Self::VariableName => {
                matches!(token.kind, Kind::Identifier(..))
            }
            Self::FunctionDeclaration => matches!(token.kind, Kind::Keyword(Keyword::Fn)),
            Self::VariableDeclaration => matches!(token.kind, Kind::Keyword(Keyword::Let)),
            Self::VariableVariants => matches!(token.kind, Kind::String(..) | Kind::Number(..)),
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

use crate::*;
use lexer::Token;

impl From<&Token> for SrcLink {
    fn from(token: &Token) -> Self {
        Self {
            from: token.pos.from,
            to: token.pos.to,
            src: token.src.to_owned(),
        }
    }
}

impl From<(&Token, &Token)> for SrcLink {
    fn from((from, to): (&Token, &Token)) -> Self {
        Self {
            from: from.pos.from,
            to: to.pos.to,
            src: from.src.to_owned(),
        }
    }
}

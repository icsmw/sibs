use crate::*;
use lexer::{KindId, Token};

impl Interest for Return {
    fn interest_in_token(token: &Token, nodes: &Nodes) -> bool {
        matches!(token.id(), KindId::Return)
    }
}

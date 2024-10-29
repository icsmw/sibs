use crate::*;
use lexer::Token;

pub trait Interest {
    fn intrested(&self, token: &Token, nodes: &Nodes) -> bool;
}

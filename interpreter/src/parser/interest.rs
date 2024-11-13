use lexer::Token;

pub trait Interest {
    fn intrested(&self, token: &Token) -> bool;
}

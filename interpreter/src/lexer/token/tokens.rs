use crate::lexer::*;

#[derive(Debug, Default)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

impl Tokens {
    pub fn with(tokens: Vec<Token>) -> Self {
        Tokens { tokens }
    }
    pub fn add(&mut self, token: Token) {
        self.tokens.push(token);
    }
    pub fn is_nl(&self) -> bool {
        let mut nl = false;
        for tk in self.tokens.iter().rev() {
            if matches!(tk.id(), KindId::Whitespace) {
                continue;
            } else if matches!(
                tk.id(),
                KindId::LF | KindId::CR | KindId::CRLF | KindId::BOF
            ) {
                nl = true;
                break;
            } else {
                break;
            }
        }
        nl
    }
    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }
    pub fn count(&self) -> usize {
        self.tokens.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.tokens.iter()
    }
}

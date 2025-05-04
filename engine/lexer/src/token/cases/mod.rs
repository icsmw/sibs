#[cfg(any(test, feature = "proptests"))]
pub mod proptests;

use crate::*;

pub mod concatenation;

#[enum_ids::enum_ids(iterator)]
enum Cases {
    Concatenation,
}

impl Cases {
    pub fn check(&self, lx: &mut Lexer, token: Token) -> Result<Option<Vec<Token>>, E> {
        match self {
            Self::Concatenation => concatenation::check(lx, token),
        }
    }
}

pub fn check(lx: &mut Lexer, token: &Token) -> Result<Option<Vec<Token>>, E> {
    for case in Cases::as_vec() {
        if let Some(tokens) = case.check(lx, token.clone())? {
            return Ok(Some(tokens));
        }
    }
    Ok(None)
}

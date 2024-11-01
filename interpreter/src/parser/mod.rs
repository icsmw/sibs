mod conflict;
mod interest;
mod nodes;
mod read;

pub use conflict::*;
pub use interest::*;
pub use nodes::*;
pub use read::*;

use lexer::{KindId, Token};

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
}

impl Parser {
    pub(crate) fn token(&mut self) -> Option<&Token> {
        while let Some(tk) = self.tokens.get(self.pos) {
            if tk.id() != KindId::Whitespace {
                return Some(tk);
            }
            self.pos += 1;
        }
        None
    }

    pub(crate) fn advance(&mut self) {
        self.pos += 1
    }

    pub(crate) fn pin(&mut self) -> impl Fn(&mut Parser) -> usize {
        let pos = self.pos;
        move |parser: &mut Parser| {
            let to_restore = parser.pos;
            parser.pos = pos;
            to_restore
        }
    }
}

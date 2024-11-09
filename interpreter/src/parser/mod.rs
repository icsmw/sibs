mod conflict;
mod interest;
mod nodes;
mod read;

pub use conflict::*;
pub use interest::*;
pub use nodes::*;
pub use read::*;

use crate::*;
use lexer::{KindId, Token};
use std::fmt;

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub(crate) fn token(&mut self) -> Option<&Token> {
        while let Some(tk) = self.tokens.get(self.pos) {
            if !matches!(tk.id(), KindId::Whitespace | KindId::BOF | KindId::EOF) {
                self.pos += 1;
                return Some(tk);
            }
            self.pos += 1;
        }
        None
    }

    pub(crate) fn tokens(&mut self, nm: usize) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(tk) = self.token().cloned() {
            tokens.push(tk);
            if tokens.len() == nm {
                return Some(tokens);
            }
        }
        None
    }

    pub(crate) fn is_next(&mut self, kind: KindId) -> bool {
        let restore = self.pin();
        let tk = self.token().cloned();
        restore(self);
        if let Some(tk) = tk {
            return tk.id() == kind;
        }
        false
    }

    pub(crate) fn pin(&mut self) -> impl Fn(&mut Parser) -> usize {
        let pos = self.pos;
        move |parser: &mut Parser| {
            let to_restore = parser.pos;
            parser.pos = pos;
            to_restore
        }
    }

    pub(crate) fn between(&mut self, left: KindId, right: KindId) -> Result<Option<Parser>, E> {
        let Some(tk) = self.token() else {
            return Ok(None);
        };
        if tk.id() != left {
            return Ok(None);
        }
        let mut tokens = Vec::new();
        let mut inside = 0;
        let closed = loop {
            let Some(tk) = self.token() else {
                break inside == 0;
            };
            if tk.id() == left {
                inside += 1;
                tokens.push(tk.clone());
                continue;
            }
            if tk.id() == right {
                if inside == 0 {
                    break true;
                } else {
                    inside -= 1;
                    tokens.push(tk.clone());
                    continue;
                }
            }
            tokens.push(tk.clone());
        };
        if closed {
            Ok(Some(Parser::new(tokens)))
        } else {
            Err(E::NoClosing(right))
        }
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tokens[self.pos..]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

mod conflict;
mod interest;
mod nodes;
mod read;

pub use conflict::*;
pub use interest::*;
pub use nodes::*;
pub use read::*;
use uuid::Uuid;

use crate::*;
use lexer::{KindId, Token};
use std::fmt;

#[derive(Debug)]
pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
    pub(crate) src: Uuid,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>, src: &Uuid) -> Self {
        Self {
            tokens,
            pos: 0,
            src: *src,
        }
    }

    pub(crate) fn token(&mut self) -> Option<&Token> {
        while let Some(tk) = self.tokens.get(self.pos) {
            if !matches!(
                tk.id(),
                KindId::Whitespace
                    | KindId::BOF
                    | KindId::EOF
                    | KindId::LF
                    | KindId::CR
                    | KindId::CRLF
            ) {
                self.pos += 1;
                return Some(tk);
            }
            self.pos += 1;
        }
        None
    }

    pub(crate) fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos).or_else(|| self.tokens.last())
    }

    pub(crate) fn until_end(&self) -> Option<(&Token, &Token)> {
        if let (Some(from), Some(to)) = (
            self.tokens.get(self.pos).or_else(|| self.tokens.last()),
            self.tokens.last(),
        ) {
            Some((from, to))
        } else {
            None
        }
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

    pub(crate) fn between(
        &mut self,
        left: KindId,
        right: KindId,
    ) -> Result<Option<(Parser, Token, Token)>, LinkedErr<E>> {
        let Some(open_tk) = self.token().cloned() else {
            return Ok(None);
        };
        if open_tk.id() != left {
            return Ok(None);
        }
        let mut tokens = Vec::new();
        let mut inside = 0;
        let close_tk = loop {
            let Some(tk) = self.token() else {
                break None;
            };
            if tk.id() == left {
                inside += 1;
                tokens.push(tk.clone());
                continue;
            }
            if tk.id() == right {
                if inside == 0 {
                    break Some(tk.to_owned());
                } else {
                    inside -= 1;
                    tokens.push(tk.clone());
                    continue;
                }
            }
            tokens.push(tk.clone());
        };
        let Some(close_tk) = close_tk else {
            return Err(LinkedErr::token(E::NoClosing(right), &open_tk));
        };
        Ok(Some((Parser::new(tokens, &self.src), open_tk, close_tk)))
    }

    pub(crate) fn is_done(&mut self) -> bool {
        let restore = self.pin();
        let is_done = self.token().is_none();
        restore(self);
        is_done
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

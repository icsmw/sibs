use crate::*;
use lexer::Token;
use parser::Parser;
use std::fmt;

#[derive(Clone, Debug)]
pub struct LinkedErr<T: Clone + fmt::Display> {
    pub link: SrcLink,
    pub e: T,
}

impl<T: Clone + fmt::Display> LinkedErr<T> {
    pub fn token(err: T, token: &Token) -> Self {
        Self {
            link: token.into(),
            e: err,
        }
    }
    pub fn between(err: T, from: &Token, to: &Token) -> Self {
        Self {
            link: (from, to).into(),
            e: err,
        }
    }
    pub fn current(err: T, parser: &Parser) -> Self {
        Self {
            link: parser
                .current()
                .map(|tk| tk.into())
                .unwrap_or(SrcLink::new(0, 0, &parser.src)),
            e: err,
        }
    }
    pub fn until_end(err: T, parser: &Parser) -> Self {
        Self {
            link: parser
                .until_end()
                .map(|tks| tks.into())
                .unwrap_or(SrcLink::new(0, 0, &parser.src)),
            e: err,
        }
    }
    pub fn by_link(err: T, link: &SrcLink) -> Self {
        Self {
            link: link.to_owned(),
            e: err,
        }
    }
}

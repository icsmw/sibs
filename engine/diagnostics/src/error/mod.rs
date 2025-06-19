mod code;

pub use code::*;

use asttree::SrcLinking;
use lexer::{LinkedPosition, Token};
use std::fmt;

#[derive(Clone, Debug)]
pub struct LinkedErr<T: fmt::Display> {
    pub link: LinkedPosition,
    pub e: T,
}

impl<T: fmt::Display> LinkedErr<T> {
    pub fn from<N: SrcLinking>(err: T, n: &N) -> Self {
        Self {
            link: (&n.link()).into(),
            e: err,
        }
    }
    pub fn sfrom<N: SrcLinking>(err: T, n: &N) -> Self {
        Self {
            link: (&n.slink()).into(),
            e: err,
        }
    }
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

    pub fn by_link(err: T, link: LinkedPosition) -> Self {
        Self { link, e: err }
    }
}

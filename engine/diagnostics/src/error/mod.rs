mod code;

pub use code::*;

use asttree::SrcLinking;
use lexer::{LinkedPosition, Token};
use std::{
    fmt::{self, Display},
    io,
};

use crate::Diagnostics;

#[derive(Clone, Debug)]
pub struct LinkedErr<E: fmt::Display + ErrorCode> {
    pub link: LinkedPosition,
    pub e: E,
}

impl<E: fmt::Display + ErrorCode> LinkedErr<E> {
    pub fn report(
        &self,
        diagnostics: &Diagnostics<E>,
        dest: &mut impl io::Write,
    ) -> Result<(), std::io::Error> {
        diagnostics.err(self, dest)
    }
    pub fn from<N: SrcLinking>(err: E, n: &N) -> Self {
        Self {
            link: (&n.link()).into(),
            e: err,
        }
    }
    pub fn sfrom<N: SrcLinking>(err: E, n: &N) -> Self {
        Self {
            link: (&n.slink()).into(),
            e: err,
        }
    }
    pub fn token(err: E, token: &Token) -> Self {
        Self {
            link: token.into(),
            e: err,
        }
    }
    pub fn between(err: E, from: &Token, to: &Token) -> Self {
        Self {
            link: (from, to).into(),
            e: err,
        }
    }

    pub fn by_link(err: E, link: LinkedPosition) -> Self {
        Self { link, e: err }
    }

    pub fn unlinked(err: E) -> Self {
        Self {
            link: LinkedPosition::default(),
            e: err,
        }
    }
}

impl<E: fmt::Display + ErrorCode> Display for LinkedErr<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.e)
    }
}

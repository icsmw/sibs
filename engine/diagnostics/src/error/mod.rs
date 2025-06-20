mod code;

pub use code::*;

use asttree::SrcLinking;
use lexer::{LinkedPosition, Token};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorStamp {
    code: &'static str,
    source: ErrorSource,
    src: Uuid,
    from: usize,
    to: usize,
}

impl<E: fmt::Display + ErrorCode> From<&LinkedErr<E>> for ErrorStamp {
    fn from(err: &LinkedErr<E>) -> Self {
        ErrorStamp {
            code: err.e.code(),
            source: err.e.src(),
            src: err.link.src,
            from: err.link.from.abs,
            to: err.link.to.abs,
        }
    }
}
#[derive(Clone, Debug)]
pub struct LinkedErr<E: fmt::Display + ErrorCode> {
    pub link: LinkedPosition,
    pub e: E,
}

impl<E: fmt::Display + ErrorCode> LinkedErr<E> {
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

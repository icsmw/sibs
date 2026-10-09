mod code;

pub use code::*;

use asttree::SrcLinking;
use lexer::{LinkedPosition, Token};
use std::{
    fmt::{self, Display},
    io,
};

use crate::{Diagnostics, DiagnosticsError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    #[default]
    Error,
    Warning,
}

impl Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Error => "error",
            Self::Warning => "warning",
        })
    }
}

#[derive(Clone, Debug)]
pub struct LinkedErr<E: fmt::Display + ErrorCode> {
    pub link: LinkedPosition,
    pub e: E,
    pub severity: Severity,
}

impl<E: fmt::Display + ErrorCode> LinkedErr<E> {
    /// Marks a diagnostic for reporting without interrupting analysis.
    /// Warnings must be collected through the context, not returned as `Err`.
    pub fn warning(mut self) -> Self {
        self.severity = Severity::Warning;
        self
    }

    pub fn map<O: Display + ErrorCode>(self, map: impl FnOnce(E) -> O) -> LinkedErr<O> {
        LinkedErr {
            link: self.link,
            e: map(self.e),
            severity: self.severity,
        }
    }

    pub fn report(
        &self,
        diagnostics: &Diagnostics<E>,
        dest: &mut impl io::Write,
    ) -> Result<(), DiagnosticsError> {
        diagnostics.err(self, dest)
    }
    pub fn from<N: SrcLinking>(err: E, n: &N) -> Self {
        Self {
            link: (&n.link()).into(),
            e: err,
            severity: Severity::Error,
        }
    }
    pub fn sfrom<N: SrcLinking>(err: E, n: &N) -> Self {
        Self {
            link: (&n.slink()).into(),
            e: err,
            severity: Severity::Error,
        }
    }
    pub fn token(err: E, token: &Token) -> Self {
        Self {
            link: token.into(),
            e: err,
            severity: Severity::Error,
        }
    }
    pub fn between(err: E, from: &Token, to: &Token) -> Self {
        Self {
            link: (from, to).into(),
            e: err,
            severity: Severity::Error,
        }
    }

    pub fn by_link(err: E, link: LinkedPosition) -> Self {
        Self {
            link,
            e: err,
            severity: Severity::Error,
        }
    }

    pub fn unlinked(err: E) -> Self {
        Self {
            link: LinkedPosition::default(),
            e: err,
            severity: Severity::Error,
        }
    }
}

impl<E: fmt::Display + ErrorCode> Display for LinkedErr<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.e)
    }
}

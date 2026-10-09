use std::{collections::HashSet, fmt::Display};
use uuid::Uuid;

use crate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorStamp {
    severity: Severity,
    code: &'static str,
    source: ErrorSource,
    src: Uuid,
    from: usize,
    to: usize,
}

impl<E: Display + ErrorCode> From<&LinkedErr<E>> for ErrorStamp {
    fn from(err: &LinkedErr<E>) -> Self {
        ErrorStamp {
            severity: err.severity,
            code: err.e.code(),
            source: err.e.src(),
            src: err.link.src,
            from: err.link.from.abs,
            to: err.link.to.abs,
        }
    }
}

#[derive(Debug)]
pub struct Errors<E: Display + ErrorCode> {
    errors: Vec<LinkedErr<E>>,
    stamps: HashSet<ErrorStamp>,
}

impl<E: Display + ErrorCode> Errors<E> {
    pub fn push(&mut self, err: LinkedErr<E>) {
        let stamp: ErrorStamp = (&err).into();
        if self.stamps.insert(stamp) {
            self.errors.push(err);
        }
    }
    pub fn take_first(&mut self) -> Option<LinkedErr<E>> {
        if self.errors.is_empty() {
            None
        } else {
            let err = self.errors.remove(0);
            self.stamps.remove(&ErrorStamp::from(&err));
            Some(err)
        }
    }
    pub fn take_first_error(&mut self) -> Option<LinkedErr<E>> {
        let index = self
            .errors
            .iter()
            .position(|err| err.severity == Severity::Error)?;
        let err = self.errors.remove(index);
        self.stamps.remove(&ErrorStamp::from(&err));
        Some(err)
    }
    pub fn has_errors(&self) -> bool {
        self.errors
            .iter()
            .any(|err| err.severity == Severity::Error)
    }
    pub fn has_warnings(&self) -> bool {
        self.errors
            .iter()
            .any(|err| err.severity == Severity::Warning)
    }
    pub fn slice(&self) -> &[LinkedErr<E>] {
        &self.errors
    }
    pub fn transform<O: Display + ErrorCode>(
        self,
        mut map: impl FnMut(LinkedErr<E>) -> LinkedErr<O>,
    ) -> Errors<O> {
        let mut transformed = Errors::default();
        for err in self.errors {
            transformed.push(map(err));
        }
        transformed
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    #[must_use]
    pub fn drain(&mut self) -> Vec<LinkedErr<E>> {
        self.stamps.clear();
        std::mem::take(&mut self.errors)
    }
}

impl<E: Display + ErrorCode> Default for Errors<E> {
    fn default() -> Self {
        Self {
            errors: Vec::new(),
            stamps: HashSet::new(),
        }
    }
}

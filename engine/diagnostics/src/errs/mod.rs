use std::{collections::HashMap, fmt::Display};

use crate::*;

#[derive(Debug)]
pub struct Errors<E: Display + ErrorCode> {
    errors: HashMap<ErrorStamp, LinkedErr<E>>,
    first: Option<ErrorStamp>,
}

impl<E: Display + ErrorCode> Errors<E> {
    pub fn add(&mut self, err: LinkedErr<E>) {
        if self.first.is_none() {
            self.first = Some((&err).into());
        }
        let stamp: ErrorStamp = (&err).into();
        self.errors.entry(stamp).or_insert(err);
    }
    pub fn extract_first(&mut self) -> Option<LinkedErr<E>> {
        self.first
            .as_ref()
            .and_then(|stamp| self.errors.remove(stamp))
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    #[must_use]
    pub fn drain(&mut self) -> Vec<LinkedErr<E>> {
        self.errors.drain().map(|(_, err)| err).collect()
    }
}

impl<E: Display + ErrorCode> Default for Errors<E> {
    fn default() -> Self {
        Self {
            errors: HashMap::new(),
            first: None,
        }
    }
}

use std::fmt::Display;

use crate::*;

#[derive(Debug)]
pub struct Errors<E: Display + ErrorCode> {
    pub errors: Vec<LinkedErr<E>>,
}

impl<E: Display + ErrorCode> Errors<E> {
    pub fn add(&mut self, err: LinkedErr<E>) {
        self.errors.push(err);
    }
    pub fn first(&mut self) -> Option<LinkedErr<E>> {
        if self.errors.is_empty() {
            None
        } else {
            Some(self.errors.remove(0))
        }
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    pub fn extract(&mut self) -> Vec<LinkedErr<E>> {
        self.errors.drain(..).collect()
    }
}

impl<E: Display + ErrorCode> Default for Errors<E> {
    fn default() -> Self {
        Self { errors: Vec::new() }
    }
}

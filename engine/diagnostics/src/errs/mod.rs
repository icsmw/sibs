use std::fmt::Display;

use crate::LinkedErr;

#[derive(Debug)]
pub struct Errors<E: Display> {
    pub errors: Vec<LinkedErr<E>>,
}

impl<E: Display> Errors<E> {
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
}

impl<E: Display> Default for Errors<E> {
    fn default() -> Self {
        Self { errors: Vec::new() }
    }
}

use std::slice;

use crate::*;

pub struct ErrorsIterator<'a> {
    errors: slice::Iter<'a, LinkedErr<DiagnosticError>>,
    ctx: &'a InterContext,
}

impl<'a> ErrorsIterator<'a> {
    pub fn new(errors: &'a [LinkedErr<DiagnosticError>], ctx: &'a InterContext) -> Self {
        Self {
            errors: errors.iter(),
            ctx,
        }
    }
}

pub struct ErrorLocator<'a> {
    pub err: &'a LinkedErr<DiagnosticError>,
    pub locator: LocationIterator<'a>,
}

impl<'a> ErrorLocator<'a> {
    pub fn new(err: &'a LinkedErr<DiagnosticError>, locator: LocationIterator<'a>) -> Self {
        Self { err, locator }
    }
}

impl<'a> Iterator for ErrorsIterator<'a> {
    type Item = ErrorLocator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let err = self.errors.next()?;
        let link = &err.link;
        Some(ErrorLocator::new(
            err,
            LocationIterator::new(link.src, link.from.abs, self.ctx),
        ))
    }
}

#[cfg(test)]
mod tests;

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
        let idx = self
            .ctx
            .get_diagnostics()
            .and_then(|diagnostics| diagnostics.tokens().get(&link.src))
            .map(|tokens| {
                tokens
                    .get_by_pos(link.from.abs)
                    .map(|(_, idx)| idx)
                    .or_else(|| {
                        tokens
                            .iter()
                            .position(|token| token.pos.from.abs >= link.from.abs)
                    })
                    .unwrap_or(tokens.count())
            })
            .unwrap_or(0);
        Some(ErrorLocator::new(
            err,
            LocationIterator::new(link.src, idx, self.ctx),
        ))
    }
}

#[cfg(test)]
mod tests;

use std::collections::HashSet;

use diagnostics::{Errors, LinkedErr};
use uuid::Uuid;

use crate::LintError;

#[derive(Debug, Default)]
pub struct LintContext {
    pub errs: Errors<LintError>,
    pub(crate) visited: HashSet<Uuid>,
}

impl LintContext {
    pub fn report(&mut self, diagnostic: LinkedErr<LintError>) {
        self.errs.push(diagnostic);
    }
}

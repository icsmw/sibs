mod ast;
mod context;
mod error;

pub(crate) use asttree::*;
pub use context::LintContext;
use diagnostics::LinkedErr;
pub use error::E as LintError;
pub(crate) use error::*;

/// Local checks for a node. Child traversal is handled by `LinkedNode`.
/// Report findings through the context; warnings do not interrupt the pass.
pub trait Lint {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<LintError>>;
}

/// Checks a parsed tree without requiring semantic initialization or execution.
pub fn check(root: &LinkedNode) -> diagnostics::Errors<LintError> {
    let mut ctx = LintContext::default();
    if let Err(err) = root.lint(root.get_md(), &mut ctx) {
        ctx.report(err);
    }
    ctx.errs
}

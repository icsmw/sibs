mod gatekeeper;
mod skip;

use crate::*;

impl Lint for ControlFlowModifier {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.lint(md, ctx),
            ControlFlowModifier::Skip(n) => n.lint(md, ctx),
        }
    }
}

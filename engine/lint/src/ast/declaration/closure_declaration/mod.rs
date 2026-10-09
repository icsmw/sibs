use crate::*;

impl Lint for ClosureDeclaration {
    fn lint(&self, _md: &Metadata, _ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

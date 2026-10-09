use crate::*;

impl Lint for BinaryExp {
    fn lint(&self, _md: &Metadata, _ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

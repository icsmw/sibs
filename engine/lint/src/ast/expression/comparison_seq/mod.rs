use crate::*;

impl Lint for ComparisonSeq {
    fn lint(&self, _md: &Metadata, _ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

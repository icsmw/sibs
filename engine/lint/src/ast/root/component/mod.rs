use crate::*;

impl Lint for Component {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        if !md.doc_lines().any(|line| !line.trim().is_empty()) {
            ctx.report(
                LinkedErr::token(E::MissingComponentDocs(self.get_name()), &self.name).warning(),
            );
        }
        Ok(())
    }
}

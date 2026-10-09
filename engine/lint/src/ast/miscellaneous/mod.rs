mod comment;
mod meta;
mod root_meta;

use crate::*;

impl Lint for Miscellaneous {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.lint(md, ctx),
            Miscellaneous::Meta(n) => n.lint(md, ctx),
            Miscellaneous::RootMeta(n) => n.lint(md, ctx),
        }
    }
}

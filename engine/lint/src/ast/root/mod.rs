mod anchor;
mod component;
mod envs;
mod globals;
mod module;
mod task;

use crate::*;

impl Lint for Root {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            Root::GlobalsModule(n) => n.lint(md, ctx),
            Root::EnvsModule(n) => n.lint(md, ctx),
            Root::Task(n) => n.lint(md, ctx),
            Root::Component(n) => n.lint(md, ctx),
            Root::Module(n) => n.lint(md, ctx),
            Root::Anchor(n) => n.lint(md, ctx),
        }
    }
}

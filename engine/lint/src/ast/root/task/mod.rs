#[cfg(test)]
mod tests;

mod documentation;

use self::documentation::documented_arguments;
use crate::*;

impl Lint for Task {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        fn has_documentation(md: &Metadata) -> bool {
            md.doc_lines().any(|line| !line.trim().is_empty())
        }
        if !has_documentation(md) {
            ctx.report(LinkedErr::token(E::MissingTaskDocs(self.get_name()), &self.name).warning());
            return Ok(());
        }
        if self.args.is_empty() {
            return Ok(());
        }
        let markdown = md.doc_lines().collect::<Vec<_>>().join("\n");
        let documented = documented_arguments(&markdown);
        for node in &self.args {
            let Some(arg) = node.extract::<ArgumentDeclaration>() else {
                continue;
            };
            let Some(name) = arg.get_var_name() else {
                continue;
            };
            if !has_documentation(node.get_md()) && !documented.contains(name.as_str()) {
                ctx.report(
                    LinkedErr::from(
                        E::MissingTaskArgumentDocs(self.get_name(), name),
                        arg.variable.as_ref(),
                    )
                    .warning(),
                );
            }
        }
        Ok(())
    }
}

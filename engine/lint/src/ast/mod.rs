mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;

impl Lint for LinkedNode {
    fn lint(&self, _md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        // Imported module bodies are shared between their import declarations.
        if !ctx.visited.insert(*self.uuid()) {
            return Ok(());
        }
        self.get_node().lint(self.get_md(), ctx)?;
        // Includes metadata and postfix nodes, as well as structural children.
        for child in self.childs() {
            child.lint(child.get_md(), ctx)?;
        }
        Ok(())
    }
}

impl Lint for Node {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.lint(md, ctx),
            Node::Declaration(n) => n.lint(md, ctx),
            Node::Expression(n) => n.lint(md, ctx),
            Node::Miscellaneous(n) => n.lint(md, ctx),
            Node::Root(n) => n.lint(md, ctx),
            Node::Statement(n) => n.lint(md, ctx),
            Node::Value(n) => n.lint(md, ctx),
        }
    }
}

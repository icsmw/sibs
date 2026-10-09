mod arg_assignation;
mod arg_assigned_value;
mod assignation;
mod assigned_value;
mod block;
mod r#break;
mod r#for;
mod r#if;
mod join;
mod r#loop;
mod oneof;
mod optional;
mod r#return;
mod r#while;

use crate::*;

impl Lint for Statement {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.lint(md, ctx),
            Statement::AssignedValue(n) => n.lint(md, ctx),
            Statement::ArgumentAssignation(n) => n.lint(md, ctx),
            Statement::ArgumentAssignedValue(n) => n.lint(md, ctx),
            Statement::Block(n) => n.lint(md, ctx),
            Statement::Break(n) => n.lint(md, ctx),
            Statement::For(n) => n.lint(md, ctx),
            Statement::If(n) => n.lint(md, ctx),
            Statement::Join(n) => n.lint(md, ctx),
            Statement::Loop(n) => n.lint(md, ctx),
            Statement::OneOf(n) => n.lint(md, ctx),
            Statement::Optional(n) => n.lint(md, ctx),
            Statement::Return(n) => n.lint(md, ctx),
            Statement::While(n) => n.lint(md, ctx),
        }
    }
}

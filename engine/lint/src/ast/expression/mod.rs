mod accessor;
mod binary_exp;
mod binary_exp_group;
mod binary_exp_seq;
mod binary_op;
mod call;
mod command;
mod comparison;
mod comparison_group;
mod comparison_op;
mod comparison_seq;
mod compound_assignments;
mod compound_assignments_op;
mod function_call;
mod logical_op;
mod range;
mod task_call;
mod variable;

use crate::*;

impl Lint for Expression {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.lint(md, ctx),
            Expression::BinaryExp(n) => n.lint(md, ctx),
            Expression::BinaryExpGroup(n) => n.lint(md, ctx),
            Expression::BinaryExpSeq(n) => n.lint(md, ctx),
            Expression::BinaryOp(n) => n.lint(md, ctx),
            Expression::Call(n) => n.lint(md, ctx),
            Expression::Command(n) => n.lint(md, ctx),
            Expression::Comparison(n) => n.lint(md, ctx),
            Expression::ComparisonGroup(n) => n.lint(md, ctx),
            Expression::ComparisonOp(n) => n.lint(md, ctx),
            Expression::ComparisonSeq(n) => n.lint(md, ctx),
            Expression::CompoundAssignments(n) => n.lint(md, ctx),
            Expression::CompoundAssignmentsOp(n) => n.lint(md, ctx),
            Expression::FunctionCall(n) => n.lint(md, ctx),
            Expression::LogicalOp(n) => n.lint(md, ctx),
            Expression::Range(n) => n.lint(md, ctx),
            Expression::TaskCall(n) => n.lint(md, ctx),
            Expression::Variable(n) => n.lint(md, ctx),
        }
    }
}

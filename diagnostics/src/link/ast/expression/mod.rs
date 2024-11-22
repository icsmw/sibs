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
use asttree::*;

impl From<&Expression> for SrcLink {
    fn from(node: &Expression) -> Self {
        match node {
            Expression::Accessor(n) => n.into(),
            Expression::BinaryExp(n) => n.into(),
            Expression::BinaryExpGroup(n) => n.into(),
            Expression::BinaryExpSeq(n) => n.into(),
            Expression::BinaryOp(n) => n.into(),
            Expression::Call(n) => n.into(),
            Expression::Command(n) => n.into(),
            Expression::Comparison(n) => n.into(),
            Expression::ComparisonGroup(n) => n.into(),
            Expression::ComparisonOp(n) => n.into(),
            Expression::ComparisonSeq(n) => n.into(),
            Expression::CompoundAssignments(n) => n.into(),
            Expression::CompoundAssignmentsOp(n) => n.into(),
            Expression::FunctionCall(n) => n.into(),
            Expression::LogicalOp(n) => n.into(),
            Expression::Range(n) => n.into(),
            Expression::TaskCall(n) => n.into(),
            Expression::Variable(n) => n.into(),
        }
    }
}

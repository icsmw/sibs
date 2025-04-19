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

impl InferType for Expression {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.infer_type(scx),
            Expression::BinaryExp(n) => n.infer_type(scx),
            Expression::BinaryExpGroup(n) => n.infer_type(scx),
            Expression::BinaryExpSeq(n) => n.infer_type(scx),
            Expression::BinaryOp(n) => n.infer_type(scx),
            Expression::Call(n) => n.infer_type(scx),
            Expression::Command(n) => n.infer_type(scx),
            Expression::Comparison(n) => n.infer_type(scx),
            Expression::ComparisonGroup(n) => n.infer_type(scx),
            Expression::ComparisonOp(n) => n.infer_type(scx),
            Expression::ComparisonSeq(n) => n.infer_type(scx),
            Expression::CompoundAssignments(n) => n.infer_type(scx),
            Expression::CompoundAssignmentsOp(n) => n.infer_type(scx),
            Expression::FunctionCall(n) => n.infer_type(scx),
            Expression::LogicalOp(n) => n.infer_type(scx),
            Expression::Range(n) => n.infer_type(scx),
            Expression::TaskCall(n) => n.infer_type(scx),
            Expression::Variable(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for Expression {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.initialize(scx),
            Expression::BinaryExp(n) => n.initialize(scx),
            Expression::BinaryExpGroup(n) => n.initialize(scx),
            Expression::BinaryExpSeq(n) => n.initialize(scx),
            Expression::BinaryOp(n) => n.initialize(scx),
            Expression::Call(n) => n.initialize(scx),
            Expression::Command(n) => n.initialize(scx),
            Expression::Comparison(n) => n.initialize(scx),
            Expression::ComparisonGroup(n) => n.initialize(scx),
            Expression::ComparisonOp(n) => n.initialize(scx),
            Expression::ComparisonSeq(n) => n.initialize(scx),
            Expression::CompoundAssignments(n) => n.initialize(scx),
            Expression::CompoundAssignmentsOp(n) => n.initialize(scx),
            Expression::FunctionCall(n) => n.initialize(scx),
            Expression::LogicalOp(n) => n.initialize(scx),
            Expression::Range(n) => n.initialize(scx),
            Expression::TaskCall(n) => n.initialize(scx),
            Expression::Variable(n) => n.initialize(scx),
        }
    }
}

impl Finalization for Expression {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.finalize(scx),
            Expression::BinaryExp(n) => n.finalize(scx),
            Expression::BinaryExpGroup(n) => n.finalize(scx),
            Expression::BinaryExpSeq(n) => n.finalize(scx),
            Expression::BinaryOp(n) => n.finalize(scx),
            Expression::Call(n) => n.finalize(scx),
            Expression::Command(n) => n.finalize(scx),
            Expression::Comparison(n) => n.finalize(scx),
            Expression::ComparisonGroup(n) => n.finalize(scx),
            Expression::ComparisonOp(n) => n.finalize(scx),
            Expression::ComparisonSeq(n) => n.finalize(scx),
            Expression::CompoundAssignments(n) => n.finalize(scx),
            Expression::CompoundAssignmentsOp(n) => n.finalize(scx),
            Expression::FunctionCall(n) => n.finalize(scx),
            Expression::LogicalOp(n) => n.finalize(scx),
            Expression::Range(n) => n.finalize(scx),
            Expression::TaskCall(n) => n.finalize(scx),
            Expression::Variable(n) => n.finalize(scx),
        }
    }
}

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
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.infer_type(tcx),
            Expression::BinaryExp(n) => n.infer_type(tcx),
            Expression::BinaryExpGroup(n) => n.infer_type(tcx),
            Expression::BinaryExpSeq(n) => n.infer_type(tcx),
            Expression::BinaryOp(n) => n.infer_type(tcx),
            Expression::Call(n) => n.infer_type(tcx),
            Expression::Command(n) => n.infer_type(tcx),
            Expression::Comparison(n) => n.infer_type(tcx),
            Expression::ComparisonGroup(n) => n.infer_type(tcx),
            Expression::ComparisonOp(n) => n.infer_type(tcx),
            Expression::ComparisonSeq(n) => n.infer_type(tcx),
            Expression::CompoundAssignments(n) => n.infer_type(tcx),
            Expression::CompoundAssignmentsOp(n) => n.infer_type(tcx),
            Expression::FunctionCall(n) => n.infer_type(tcx),
            Expression::LogicalOp(n) => n.infer_type(tcx),
            Expression::Range(n) => n.infer_type(tcx),
            Expression::TaskCall(n) => n.infer_type(tcx),
            Expression::Variable(n) => n.infer_type(tcx),
        }
    }
}

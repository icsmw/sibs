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

impl Interpret for Expression {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.interpret(rt),
            Expression::BinaryExp(n) => n.interpret(rt),
            Expression::BinaryExpGroup(n) => n.interpret(rt),
            Expression::BinaryExpSeq(n) => n.interpret(rt),
            Expression::BinaryOp(n) => n.interpret(rt),
            Expression::Call(n) => n.interpret(rt),
            Expression::Command(n) => n.interpret(rt),
            Expression::Comparison(n) => n.interpret(rt),
            Expression::ComparisonGroup(n) => n.interpret(rt),
            Expression::ComparisonOp(n) => n.interpret(rt),
            Expression::ComparisonSeq(n) => n.interpret(rt),
            Expression::CompoundAssignments(n) => n.interpret(rt),
            Expression::CompoundAssignmentsOp(n) => n.interpret(rt),
            Expression::FunctionCall(n) => n.interpret(rt),
            Expression::LogicalOp(n) => n.interpret(rt),
            Expression::Range(n) => n.interpret(rt),
            Expression::TaskCall(n) => n.interpret(rt),
            Expression::Variable(n) => n.interpret(rt),
        }
    }
}

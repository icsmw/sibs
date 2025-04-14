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
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.interpret(rt, cx),
            Expression::BinaryExp(n) => n.interpret(rt, cx),
            Expression::BinaryExpGroup(n) => n.interpret(rt, cx),
            Expression::BinaryExpSeq(n) => n.interpret(rt, cx),
            Expression::BinaryOp(n) => n.interpret(rt, cx),
            Expression::Call(n) => n.interpret(rt, cx),
            Expression::Command(n) => n.interpret(rt, cx),
            Expression::Comparison(n) => n.interpret(rt, cx),
            Expression::ComparisonGroup(n) => n.interpret(rt, cx),
            Expression::ComparisonOp(n) => n.interpret(rt, cx),
            Expression::ComparisonSeq(n) => n.interpret(rt, cx),
            Expression::CompoundAssignments(n) => n.interpret(rt, cx),
            Expression::CompoundAssignmentsOp(n) => n.interpret(rt, cx),
            Expression::FunctionCall(n) => n.interpret(rt, cx),
            Expression::LogicalOp(n) => n.interpret(rt, cx),
            Expression::Range(n) => n.interpret(rt, cx),
            Expression::TaskCall(n) => n.interpret(rt, cx),
            Expression::Variable(n) => n.interpret(rt, cx),
        }
    }
}

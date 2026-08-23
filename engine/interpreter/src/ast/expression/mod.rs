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
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Expression::Accessor(n) => n.interpret(env),
            Expression::BinaryExp(n) => n.interpret(env),
            Expression::BinaryExpGroup(n) => n.interpret(env),
            Expression::BinaryExpSeq(n) => n.interpret(env),
            Expression::BinaryOp(n) => n.interpret(env),
            Expression::Call(n) => n.interpret(env),
            Expression::Command(n) => n.interpret(env),
            Expression::Comparison(n) => n.interpret(env),
            Expression::ComparisonGroup(n) => n.interpret(env),
            Expression::ComparisonOp(n) => n.interpret(env),
            Expression::ComparisonSeq(n) => n.interpret(env),
            Expression::CompoundAssignments(n) => n.interpret(env),
            Expression::CompoundAssignmentsOp(n) => n.interpret(env),
            Expression::FunctionCall(n) => n.interpret(env),
            Expression::LogicalOp(n) => n.interpret(env),
            Expression::Range(n) => n.interpret(env),
            Expression::TaskCall(n) => n.interpret(env),
            Expression::Variable(n) => n.interpret(env),
        }
    }
}

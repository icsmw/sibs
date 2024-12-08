mod conflict;

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

impl AsVec<ExpressionId> for ExpressionId {
    fn as_vec() -> Vec<ExpressionId> {
        ExpressionId::as_vec()
    }
}

impl TryRead<Expression, ExpressionId> for Expression {
    fn try_read(parser: &mut Parser, id: ExpressionId) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(match id {
            ExpressionId::Variable => Variable::read_as_linked(parser)?,
            ExpressionId::Comparison => Comparison::read_as_linked(parser)?,
            ExpressionId::ComparisonSeq => ComparisonSeq::read_as_linked(parser)?,
            ExpressionId::ComparisonGroup => ComparisonGroup::read_as_linked(parser)?,
            ExpressionId::LogicalOp => LogicalOp::read_as_linked(parser)?,
            ExpressionId::ComparisonOp => ComparisonOp::read_as_linked(parser)?,
            ExpressionId::Range => Range::read_as_linked(parser)?,
            ExpressionId::BinaryExpGroup => BinaryExpGroup::read_as_linked(parser)?,
            ExpressionId::BinaryExp => BinaryExp::read_as_linked(parser)?,
            ExpressionId::BinaryExpSeq => BinaryExpSeq::read_as_linked(parser)?,
            ExpressionId::BinaryOp => BinaryOp::read_as_linked(parser)?,
            ExpressionId::Call => Call::read_as_linked(parser)?,
            ExpressionId::Accessor => Accessor::read_as_linked(parser)?,
            ExpressionId::FunctionCall => FunctionCall::read_as_linked(parser)?,
            ExpressionId::CompoundAssignments => CompoundAssignments::read_as_linked(parser)?,
            ExpressionId::CompoundAssignmentsOp => CompoundAssignmentsOp::read_as_linked(parser)?,
            ExpressionId::Command => Command::read_as_linked(parser)?,
            ExpressionId::TaskCall => TaskCall::read_as_linked(parser)?,
        })
    }
}

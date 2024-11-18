use crate::*;

impl AsVec<ExpressionId> for ExpressionId {
    fn as_vec() -> Vec<ExpressionId> {
        ExpressionId::as_vec()
    }
}

impl Read<Expression, ExpressionId> for Expression {}

impl TryRead<Expression, ExpressionId> for Expression {
    fn try_read(parser: &mut Parser, id: ExpressionId) -> Result<Option<Expression>, LinkedErr<E>> {
        Ok(match id {
            ExpressionId::Variable => Variable::read(parser)?.map(Expression::Variable),
            ExpressionId::Comparison => Comparison::read(parser)?.map(Expression::Comparison),
            ExpressionId::ComparisonSeq => {
                ComparisonSeq::read(parser)?.map(Expression::ComparisonSeq)
            }
            ExpressionId::ComparisonGroup => {
                ComparisonGroup::read(parser)?.map(Expression::ComparisonGroup)
            }
            ExpressionId::LogicalOp => LogicalOp::read(parser)?.map(Expression::LogicalOp),
            ExpressionId::ComparisonOp => ComparisonOp::read(parser)?.map(Expression::ComparisonOp),
            ExpressionId::Range => Range::read(parser)?.map(Expression::Range),
            ExpressionId::BinaryExpGroup => {
                BinaryExpGroup::read(parser)?.map(Expression::BinaryExpGroup)
            }
            ExpressionId::BinaryExp => BinaryExp::read(parser)?.map(Expression::BinaryExp),
            ExpressionId::BinaryExpSeq => BinaryExpSeq::read(parser)?.map(Expression::BinaryExpSeq),
            ExpressionId::BinaryOp => BinaryOp::read(parser)?.map(Expression::BinaryOp),
            ExpressionId::Call => Call::read(parser)?.map(Expression::Call),
            ExpressionId::Accessor => Accessor::read(parser)?.map(Expression::Accessor),
            ExpressionId::FunctionCall => FunctionCall::read(parser)?.map(Expression::FunctionCall),

            ExpressionId::CompoundAssignments => {
                CompoundAssignments::read(parser)?.map(Expression::CompoundAssignments)
            }
            ExpressionId::CompoundAssignmentsOp => {
                CompoundAssignmentsOp::read(parser)?.map(Expression::CompoundAssignmentsOp)
            }
            ExpressionId::Command => Command::read(parser)?.map(Expression::Command),
            ExpressionId::TaskCall => TaskCall::read(parser)?.map(Expression::TaskCall),
        })
    }
}

use crate::*;

impl AsVec<ExpressionId> for ExpressionId {
    fn as_vec() -> Vec<ExpressionId> {
        ExpressionId::as_vec()
    }
}

impl Read<Expression, ExpressionId> for Expression {}

impl TryRead<Expression, ExpressionId> for Expression {
    fn try_read(
        parser: &mut Parser,
        id: ExpressionId,
        nodes: &Nodes,
    ) -> Result<Option<Expression>, E> {
        Ok(match id {
            ExpressionId::Variable => Variable::read(parser, nodes)?.map(Expression::Variable),
            ExpressionId::Comparing => Comparing::read(parser, nodes)?.map(Expression::Comparing),
            ExpressionId::ComparingSeq => {
                ComparingSeq::read(parser, nodes)?.map(Expression::ComparingSeq)
            }
            ExpressionId::Condition => Condition::read(parser, nodes)?.map(Expression::Condition),
            ExpressionId::LogicalOp => LogicalOp::read(parser, nodes)?.map(Expression::LogicalOp),
            ExpressionId::ComparisonOp => {
                ComparisonOp::read(parser, nodes)?.map(Expression::ComparisonOp)
            }
            ExpressionId::Range => Range::read(parser, nodes)?.map(Expression::Range),
            ExpressionId::BinaryExp => BinaryExp::read(parser, nodes)?.map(Expression::BinaryExp),
            ExpressionId::Call => Call::read(parser, nodes)?.map(Expression::Call),
            ExpressionId::Accessor => Accessor::read(parser, nodes)?.map(Expression::Accessor),
            ExpressionId::FunctionCall => {
                FunctionCall::read(parser, nodes)?.map(Expression::FunctionCall)
            }
            ExpressionId::Incrementer => {
                Incrementer::read(parser, nodes)?.map(Expression::Incrementer)
            }
            ExpressionId::Command => Command::read(parser, nodes)?.map(Expression::Command),
            ExpressionId::TaskCall => TaskCall::read(parser, nodes)?.map(Expression::TaskCall),
        })
    }
}

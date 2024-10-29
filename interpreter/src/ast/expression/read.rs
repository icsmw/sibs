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
        })
    }
}

use crate::*;

impl ReadElement<CompoundAssignments> for CompoundAssignments {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<CompoundAssignments>, E> {
        let Some(left) =
            Expression::try_read(parser, ExpressionId::Variable, nodes)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(operator) =
            Expression::try_read(parser, ExpressionId::CompoundAssignmentsOp, nodes)?
                .map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(right) = Value::try_read(parser, ValueId::Number, nodes)?
            .map(Node::Value)
            .or(Expression::try_read(parser, ExpressionId::Variable, nodes)?.map(Node::Expression))
        else {
            return Ok(None);
        };
        Ok(Some(CompoundAssignments {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }))
    }
}

use crate::*;

impl ReadNode<CompoundAssignments> for CompoundAssignments {
    fn read(parser: &mut Parser) -> Result<Option<CompoundAssignments>, LinkedErr<E>> {
        let Some(left) =
            Expression::try_read(parser, ExpressionId::Variable)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(operator) = Expression::try_read(parser, ExpressionId::CompoundAssignmentsOp)?
            .map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(right) = Node::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[ExpressionId::Variable, ExpressionId::BinaryExpSeq]),
            ],
        )?
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

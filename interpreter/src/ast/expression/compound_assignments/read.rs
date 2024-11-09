use crate::*;

impl ReadNode<CompoundAssignments> for CompoundAssignments {
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
        let Some(right) = Node::try_oneof(
            parser,
            nodes,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
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

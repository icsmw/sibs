use crate::*;

impl ReadNode<BinaryExp> for BinaryExp {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<BinaryExp>, E> {
        let Some(left) = Node::try_oneof(
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
        let Some(operator) =
            Expression::try_read(parser, ExpressionId::BinaryOp, nodes)?.map(Node::Expression)
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
        Ok(Some(BinaryExp {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }))
    }
}

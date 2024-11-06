use crate::*;

impl ReadElement<Comparison> for Comparison {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Comparison>, E> {
        let Some(left) = Node::try_oneof(
            parser,
            nodes,
            &[
                NodeReadTarget::Value(&[ValueId::Number, ValueId::Boolean]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Ok(None);
        };
        parser.advance();
        let Some(operator) =
            Expression::try_read(parser, ExpressionId::ComparisonOp, nodes)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        parser.advance();
        let Some(right) = Node::try_oneof(
            parser,
            nodes,
            &[
                NodeReadTarget::Value(&[ValueId::Number, ValueId::Boolean]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(Comparison {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }))
    }
}

use lexer::Kind;

use crate::*;

impl ReadElement<Comparison> for Comparison {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Comparison>, E> {
        fn get_side(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Node>, E> {
            Ok(Expression::try_read(parser, ExpressionId::Variable, nodes)?
                .map(Node::Expression)
                .or(Value::try_read(parser, ValueId::Boolean, nodes)?.map(Node::Value))
                .or(Value::try_read(parser, ValueId::Number, nodes)?.map(Node::Value)))
        }

        let Some(left) = get_side(parser, nodes)? else {
            return Ok(None);
        };
        parser.advance();
        let Some(operator) =
            Expression::try_read(parser, ExpressionId::ComparisonOp, nodes)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        parser.advance();
        let Some(right) = get_side(parser, nodes)? else {
            return Ok(None);
        };
        Ok(Some(Comparison {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }))
    }
}

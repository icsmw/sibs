use lexer::Kind;

use crate::*;

impl ReadElement<Range> for Range {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Range>, E> {
        fn get_side(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Node>, E> {
            Ok(Value::try_read(parser, ValueId::Number, nodes)?
                .map(Node::Value)
                .or(Expression::try_read(parser, ExpressionId::Variable, nodes)?
                    .map(Node::Expression)))
        }
        let Some(left) = get_side(parser, nodes)? else {
            return Ok(None);
        };
        parser.advance();
        if let Some(tk) = parser.token() {
            if !matches!(tk.kind, Kind::DotDot) {
                return Ok(None);
            }
        }
        parser.advance();
        let Some(right) = get_side(parser, nodes)? else {
            return Ok(None);
        };
        Ok(Some(Range {
            left: Box::new(left),
            right: Box::new(right),
        }))
    }
}

use lexer::Kind;

use crate::*;

impl ReadNode<Call> for Call {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Call>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Dot) {
            return Ok(None);
        }
        let Some(node) =
            Expression::try_read(parser, ExpressionId::FunctionCall, nodes)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        Ok(Some(Call {
            token,
            node: Box::new(node),
        }))
    }
}

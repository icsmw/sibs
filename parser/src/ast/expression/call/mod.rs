mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use lexer::Kind;

impl ReadNode<Call> for Call {
    fn read(parser: &mut Parser) -> Result<Option<Call>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Dot) {
            return Ok(None);
        }
        let Some(node) =
            Expression::try_read(parser, ExpressionId::FunctionCall)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        Ok(Some(Call {
            token,
            node: Box::new(node),
        }))
    }
}

#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Call {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Dot)
    }
}

impl ReadNode<Call> for Call {
    fn read(parser: &mut Parser) -> Result<Option<Call>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Dot) {
            return Ok(None);
        }
        let restore = parser.pin();
        let Some(next) = parser.token() else {
            return Err(LinkedErr::token(E::MissedCallExpression, &token));
        };
        if matches!(next.kind, Kind::Keyword(..)) {
            return Err(LinkedErr::token(E::KeywordUsing, next));
        }
        restore(parser);
        let Some(node) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[ExpressionId::FunctionCall])],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(Call {
            token,
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}

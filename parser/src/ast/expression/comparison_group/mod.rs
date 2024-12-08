#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ComparisonGroup {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::LeftParen)
    }
}

impl ReadNode<ComparisonGroup> for ComparisonGroup {
    fn read(parser: &mut Parser) -> Result<Option<ComparisonGroup>, LinkedErr<E>> {
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
        };
        let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[NodeReadTarget::Expression(&[ExpressionId::ComparisonSeq])],
        )?
        else {
            return Ok(None);
        };
        Ok(if inner.is_done() {
            Some(ComparisonGroup {
                open,
                close,
                node: Box::new(node),
                uuid: Uuid::new_v4(),
            })
        } else {
            None
        })
    }
}

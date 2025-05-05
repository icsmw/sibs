#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for BinaryExpGroup {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::LeftParen)
    }
}

impl ReadNode<BinaryExpGroup> for BinaryExpGroup {
    fn read(parser: &Parser) -> Result<Option<BinaryExpGroup>, LinkedErr<E>> {
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
        };
        let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[NodeTarget::Expression(&[ExpressionId::BinaryExpSeq])],
        )?
        else {
            return Ok(None);
        };
        Ok(if inner.is_done() {
            Some(BinaryExpGroup {
                open: open.clone(),
                close: close.clone(),
                node: Box::new(node),
                uuid: Uuid::new_v4(),
            })
        } else {
            None
        })
    }
}

#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ComparisonGroup {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::LeftParen | Kind::Bang)
    }
}

impl ReadNode<ComparisonGroup> for ComparisonGroup {
    fn read(parser: &Parser) -> Result<Option<ComparisonGroup>, LinkedErr<E>> {
        let restore = parser.pin();
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        let negation = if matches!(token.kind, Kind::Bang) {
            Some(token)
        } else {
            restore(parser);
            None
        };
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
        };
        let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[NodeTarget::Expression(&[ExpressionId::ComparisonSeq])],
        )?
        else {
            return Ok(None);
        };
        Ok(if inner.is_done() {
            Some(ComparisonGroup {
                open: open.clone(),
                close: close.clone(),
                node: Box::new(node),
                negation: negation.map(|tk| tk.clone()),
                uuid: Uuid::new_v4(),
            })
        } else {
            None
        })
    }
}

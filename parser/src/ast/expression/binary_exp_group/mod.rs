#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::KindId;

impl ReadNode<BinaryExpGroup> for BinaryExpGroup {
    fn read(parser: &mut Parser) -> Result<Option<BinaryExpGroup>, LinkedErr<E>> {
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
        };
        let Some(node) =
            Expression::try_oneof(&mut inner, &[ExpressionId::BinaryExpSeq])?.map(Node::Expression)
        else {
            return Ok(None);
        };
        Ok(if inner.is_done() {
            Some(BinaryExpGroup {
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

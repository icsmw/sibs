#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Accessor {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::LeftBracket)
    }
}

impl ReadNode<Accessor> for Accessor {
    fn read(parser: &mut Parser) -> Result<Option<Accessor>, LinkedErr<E>> {
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftBracket, KindId::RightBracket)?
        else {
            return Ok(None);
        };
        let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[
                NodeTarget::Value(&[ValueId::Number]),
                NodeTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExpSeq,
                    ExpressionId::FunctionCall,
                ]),
                NodeTarget::Statement(&[StatementId::If]),
            ],
        )?
        else {
            return Ok(None);
        };
        if !inner.is_done() {
            return Ok(None);
        }
        Ok(Some(Accessor {
            node: Box::new(node),
            open,
            close,
            uuid: Uuid::new_v4(),
        }))
    }
}

#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::KindId;

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
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExpSeq,
                    ExpressionId::FunctionCall,
                ]),
                NodeReadTarget::Statement(&[StatementId::If]),
            ],
        )?
        else {
            return Ok(None);
        };
        if !parser.is_done() {
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

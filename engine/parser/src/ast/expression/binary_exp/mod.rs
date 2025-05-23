#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for BinaryExp {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Identifier(..) | Kind::Number(..) | Kind::Bang
        )
    }
}

impl ReadNode<BinaryExp> for BinaryExp {
    fn read(parser: &Parser) -> Result<Option<BinaryExp>, LinkedErr<E>> {
        let Some(left) = LinkedNode::try_oneof(
            parser,
            &[
                NodeTarget::Value(&[ValueId::Number]),
                NodeTarget::Expression(&[ExpressionId::Variable, ExpressionId::FunctionCall]),
            ],
        )?
        else {
            return Ok(None);
        };
        let Some(operator) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Expression(&[ExpressionId::BinaryOp])],
        )?
        else {
            return Ok(None);
        };
        let Some(right) = LinkedNode::try_oneof(
            parser,
            &[
                NodeTarget::Value(&[ValueId::Number]),
                NodeTarget::Expression(&[ExpressionId::Variable, ExpressionId::FunctionCall]),
            ],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(BinaryExp {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
            uuid: Uuid::new_v4(),
        }))
    }
}

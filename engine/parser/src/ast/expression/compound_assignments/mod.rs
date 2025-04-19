#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for CompoundAssignments {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..) | Kind::Bang)
    }
}

impl ReadNode<CompoundAssignments> for CompoundAssignments {
    fn read(parser: &mut Parser) -> Result<Option<CompoundAssignments>, LinkedErr<E>> {
        let Some(left) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Expression(&[ExpressionId::Variable])],
        )?
        else {
            return Ok(None);
        };
        let Some(operator) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Expression(&[
                ExpressionId::CompoundAssignmentsOp,
            ])],
        )?
        else {
            return Ok(None);
        };
        let Some(right) = LinkedNode::try_oneof(
            parser,
            &[
                NodeTarget::Value(&[ValueId::Number]),
                NodeTarget::Expression(&[ExpressionId::Variable, ExpressionId::BinaryExpSeq]),
            ],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(CompoundAssignments {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
            uuid: Uuid::new_v4(),
        }))
    }
}

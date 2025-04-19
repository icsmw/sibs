#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ArgumentAssignation {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..) | Kind::Bang)
    }
}

impl ReadNode<ArgumentAssignation> for ArgumentAssignation {
    fn read(parser: &mut Parser) -> Result<Option<ArgumentAssignation>, LinkedErr<E>> {
        let Some(left) =
            LinkedNode::try_oneof(parser, &[NodeTarget::Expression(&[ExpressionId::Variable])])?
        else {
            return Ok(None);
        };
        let Some(right) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Statement(&[StatementId::ArgumentAssignedValue])],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(ArgumentAssignation {
            left: Box::new(left),
            right: Box::new(right),
            uuid: Uuid::new_v4(),
        }))
    }
}

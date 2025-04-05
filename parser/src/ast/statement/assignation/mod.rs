#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Assignation {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..) | Kind::Bang)
    }
}

impl ReadNode<Assignation> for Assignation {
    fn read(parser: &mut Parser) -> Result<Option<Assignation>, LinkedErr<E>> {
        let Some(left) =
            LinkedNode::try_oneof(parser, &[NodeTarget::Expression(&[ExpressionId::Variable])])?
        else {
            return Ok(None);
        };
        let Some(right) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Statement(&[StatementId::AssignedValue])],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(Assignation {
            left: Box::new(left),
            right: Box::new(right),
            uuid: Uuid::new_v4(),
        }))
    }
}

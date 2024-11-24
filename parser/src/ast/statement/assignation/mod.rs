#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;

impl ReadNode<Assignation> for Assignation {
    fn read(parser: &mut Parser) -> Result<Option<Assignation>, LinkedErr<E>> {
        let Some(left) =
            Expression::try_oneof(parser, &[ExpressionId::Variable])?.map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(right) =
            Statement::try_read(parser, StatementId::AssignedValue)?.map(Node::Statement)
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

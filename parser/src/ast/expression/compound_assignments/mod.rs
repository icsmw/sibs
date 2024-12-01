#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;

impl ReadNode<CompoundAssignments> for CompoundAssignments {
    fn read(parser: &mut Parser) -> Result<Option<CompoundAssignments>, LinkedErr<E>> {
        let Some(left) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[ExpressionId::Variable])],
        )?
        else {
            return Ok(None);
        };
        let Some(operator) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[
                ExpressionId::CompoundAssignmentsOp,
            ])],
        )?
        else {
            return Ok(None);
        };
        let Some(right) = LinkedNode::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[ExpressionId::Variable, ExpressionId::BinaryExpSeq]),
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

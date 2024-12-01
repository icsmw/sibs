#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;

impl ReadNode<Comparison> for Comparison {
    fn read(parser: &mut Parser) -> Result<Option<Comparison>, LinkedErr<E>> {
        let Some(left) = LinkedNode::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[
                    ValueId::Number,
                    ValueId::Boolean,
                    ValueId::PrimitiveString,
                    ValueId::InterpolatedString,
                ]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Ok(None);
        };
        let Some(operator) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[ExpressionId::ComparisonOp])],
        )?
        else {
            return Ok(None);
        };
        let Some(right) = LinkedNode::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[
                    ValueId::Number,
                    ValueId::Boolean,
                    ValueId::PrimitiveString,
                    ValueId::InterpolatedString,
                ]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(Comparison {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
            uuid: Uuid::new_v4(),
        }))
    }
}

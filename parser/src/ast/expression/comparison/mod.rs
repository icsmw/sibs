#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Comparison {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Identifier(..)
                | Kind::Number(..)
                | Kind::String(..)
                | Kind::InterpolatedString(..)
                | Kind::Keyword(Keyword::True)
                | Kind::Keyword(Keyword::False)
                | Kind::Bang
        )
    }
}

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

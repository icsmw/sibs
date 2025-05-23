#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for AssignedValue {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Equals)
    }
}

impl ReadNode<AssignedValue> for AssignedValue {
    fn read(parser: &Parser) -> Result<Option<AssignedValue>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Equals) {
            return Ok(None);
        };
        let node = LinkedNode::try_oneof(
            parser,
            &[
                NodeTarget::Value(&[
                    ValueId::Number,
                    ValueId::Boolean,
                    ValueId::PrimitiveString,
                    ValueId::InterpolatedString,
                    ValueId::Array,
                    ValueId::Closure,
                ]),
                NodeTarget::Statement(&[StatementId::If]),
                NodeTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExpSeq,
                    ExpressionId::ComparisonSeq,
                    ExpressionId::FunctionCall,
                    ExpressionId::Command,
                    ExpressionId::TaskCall,
                    ExpressionId::Range,
                ]),
            ],
        )?
        .ok_or_else(|| E::InvalidAssignation(parser.to_string()).link_with_token(&token))?;
        Ok(Some(AssignedValue {
            token: token.clone(),
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}

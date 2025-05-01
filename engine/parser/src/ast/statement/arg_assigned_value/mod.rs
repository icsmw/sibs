#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ArgumentAssignedValue {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Equals)
    }
}

impl ReadNode<ArgumentAssignedValue> for ArgumentAssignedValue {
    fn read(parser: &Parser) -> Result<Option<ArgumentAssignedValue>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Equals) {
            return Ok(None);
        };
        let node = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Value(&[
                ValueId::Number,
                ValueId::Boolean,
                ValueId::PrimitiveString,
                ValueId::Array,
            ])],
        )?
        .ok_or_else(|| E::InvalidAssignation(parser.to_string()).link_with_token(&token))?;
        Ok(Some(ArgumentAssignedValue {
            token,
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}

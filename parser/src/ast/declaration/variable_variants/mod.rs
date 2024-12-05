#[cfg(test)]
mod proptests;

use crate::*;

impl ReadNode<VariableVariants> for VariableVariants {
    fn read(parser: &mut Parser) -> Result<Option<VariableVariants>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Colon) {
            return Ok(None);
        }
        let mut variants = Vec::new();
        loop {
            let Some(node) = LinkedNode::try_oneof(
                parser,
                &[NodeReadTarget::Value(&[
                    ValueId::PrimitiveString,
                    ValueId::Number,
                ])],
            )?
            else {
                break;
            };
            variants.push(node);
            let restore = parser.pin();
            if let Some(nx) = parser.token() {
                if !matches!(nx.kind, Kind::VerticalBar) {
                    restore(parser);
                    break;
                }
            } else {
                break;
            }
        }
        if variants.is_empty() {
            Ok(None)
        } else {
            Ok(Some(VariableVariants {
                token,
                variants,
                uuid: Uuid::new_v4(),
            }))
        }
    }
}

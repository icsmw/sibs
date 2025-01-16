#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for VariableVariants {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Colon)
    }
}

impl ReadNode<VariableVariants> for VariableVariants {
    fn read(parser: &mut Parser) -> Result<Option<VariableVariants>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Colon) {
            return Ok(None);
        }
        let mut variants = Vec::new();
        let mut vbar = None;
        loop {
            let Some(node) = LinkedNode::try_oneof(
                parser,
                &[NodeTarget::Value(&[
                    ValueId::PrimitiveString,
                    ValueId::Number,
                ])],
            )?
            else {
                break;
            };
            vbar = None;
            variants.push(node);
            let restore = parser.pin();
            if let Some(nx) = parser.token() {
                if !matches!(nx.kind, Kind::VerticalBar) {
                    restore(parser);
                    break;
                }
                vbar = Some(restore);
            } else {
                break;
            }
        }
        if let Some(restore) = vbar {
            restore(parser);
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

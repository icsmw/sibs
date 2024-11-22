#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::Kind;

impl ReadNode<VariableVariants> for VariableVariants {
    fn read(parser: &mut Parser) -> Result<Option<VariableVariants>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Colon) {
            return Ok(None);
        }
        let mut types = Vec::new();
        loop {
            let Some(node) = Node::try_oneof(
                parser,
                &[NodeReadTarget::Value(&[
                    ValueId::PrimitiveString,
                    ValueId::Number,
                ])],
            )?
            else {
                break;
            };
            types.push(node);
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
        if types.is_empty() {
            Ok(None)
        } else {
            Ok(Some(VariableVariants { token, types }))
        }
    }
}

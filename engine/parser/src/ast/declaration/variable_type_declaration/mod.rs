#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for VariableTypeDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Colon)
    }
}

impl ReadNode<VariableTypeDeclaration> for VariableTypeDeclaration {
    fn read(parser: &Parser) -> Result<Option<VariableTypeDeclaration>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Colon) {
            return Ok(None);
        }
        let mut types = Vec::new();
        let mut vbar = None;
        loop {
            let Some(node) = LinkedNode::try_oneof(
                parser,
                &[NodeTarget::Declaration(&[DeclarationId::VariableType])],
            )?
            else {
                break;
            };
            vbar = None;
            types.push(node);
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
        if types.is_empty() {
            Ok(None)
        } else {
            Ok(Some(VariableTypeDeclaration {
                token: token.clone(),
                types,
                uuid: Uuid::new_v4(),
            }))
        }
    }
}

use crate::*;
use lexer::Kind;

impl ReadNode<VariableTypeDeclaration> for VariableTypeDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<VariableTypeDeclaration>, E> {
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
                &[NodeReadTarget::Declaration(&[DeclarationId::VariableType])],
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
            Ok(Some(VariableTypeDeclaration { token, types }))
        }
    }
}

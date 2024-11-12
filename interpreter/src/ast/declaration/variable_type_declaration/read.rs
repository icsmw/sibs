use crate::*;
use lexer::Kind;

impl ReadNode<VariableTypeDeclaration> for VariableTypeDeclaration {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<VariableTypeDeclaration>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Colon) {
            return Ok(None);
        }
        let mut types = Vec::new();
        loop {
            let Some(node) = Declaration::try_read(parser, DeclarationId::VariableType, nodes)?
                .map(Node::Declaration)
            else {
                break;
            };
            types.push(node);
            if let Some(nx) = parser.token() {
                if !matches!(nx.kind, Kind::VerticalBar) {
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

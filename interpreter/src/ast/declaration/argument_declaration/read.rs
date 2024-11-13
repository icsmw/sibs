use lexer::Kind;

use crate::*;

impl ReadNode<VariableDeclaration> for VariableDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<VariableDeclaration>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Let) {
            return Ok(None);
        }
        let Some(token_ident) = parser.token() else {
            return Err(E::MissedVariableDefinition);
        };
        if let Some(tk) = parser.token() {
        } else {
        };
        let Some(node) =
            Statement::try_read(parser, StatementId::Assignation)?.map(Node::Statement)
        else {
            return Err(E::MissedVariableDefinition);
        };
        Ok(Some(VariableDeclaration {
            token,
            node: Box::new(node),
        }))
    }
}

use lexer::Kind;

use crate::*;

impl ReadNode<VariableDeclaration> for VariableDeclaration {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<VariableDeclaration>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Let) {
            return Ok(None);
        }
        let Some(variable) =
            Expression::try_read(parser, ExpressionId::Variable, nodes)?.map(Node::Expression)
        else {
            return Err(E::MissedVariableDefinition);
        };
        let ty = Node::try_oneof(
            parser,
            nodes,
            &[NodeReadTarget::Declaration(&[
                DeclarationId::VariableTypeDeclaration,
            ])],
        )?
        .map(Box::new);
        let assignation = Node::try_oneof(
            parser,
            nodes,
            &[NodeReadTarget::Statement(&[StatementId::AssignedValue])],
        )?
        .map(Box::new);
        Ok(Some(VariableDeclaration {
            token,
            variable: Box::new(variable),
            r#type: ty,
            assignation,
        }))
    }
}

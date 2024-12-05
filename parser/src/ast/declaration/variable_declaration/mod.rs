#[cfg(test)]
mod proptests;

use crate::*;

impl ReadNode<VariableDeclaration> for VariableDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<VariableDeclaration>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Let)) {
            return Ok(None);
        }
        let variable = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Declaration(&[DeclarationId::VariableName])],
        )?
        .ok_or_else(|| E::MissedVariableDefinition.link_with_token(&token))?;
        let ty = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Declaration(&[
                DeclarationId::VariableTypeDeclaration,
            ])],
        )?
        .map(Box::new);
        let assignation = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Statement(&[StatementId::AssignedValue])],
        )?
        .map(Box::new);
        Ok(Some(VariableDeclaration {
            token,
            variable: Box::new(variable),
            r#type: ty,
            assignation,
            uuid: Uuid::new_v4(),
        }))
    }
}

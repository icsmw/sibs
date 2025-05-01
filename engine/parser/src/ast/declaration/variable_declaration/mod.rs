#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for VariableDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Let))
    }
}

impl ReadNode<VariableDeclaration> for VariableDeclaration {
    fn read(parser: &Parser) -> Result<Option<VariableDeclaration>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Let)) {
            return Ok(None);
        }
        let restore = parser.pin();
        let Some(next) = parser.token() else {
            return Err(LinkedErr::token(E::MissedVariableName, &token));
        };
        if matches!(next.kind, Kind::Keyword(..)) {
            return Err(LinkedErr::token(E::KeywordUsing, next));
        }
        restore(parser);
        let variable = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Declaration(&[DeclarationId::VariableName])],
        )?
        .ok_or_else(|| E::MissedVariableDefinition.link_with_token(&token))?;
        let ty = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Declaration(&[
                DeclarationId::VariableTypeDeclaration,
            ])],
        )?
        .map(Box::new);
        let assignation = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Statement(&[StatementId::AssignedValue])],
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

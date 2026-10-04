#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for GlobalDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Global | Keyword::Const))
    }
}

impl ReadNode<GlobalDeclaration> for GlobalDeclaration {
    fn read(parser: &Parser) -> Result<Option<GlobalDeclaration>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !Self::intrested(&token) {
            return Ok(None);
        }
        let restore = parser.pin();
        let Some(next) = parser.token() else {
            return Err(LinkedErr::token(E::MissedVariableName, &token));
        };
        if matches!(next.kind, Kind::Keyword(..)) {
            return Err(LinkedErr::token(E::KeywordUsing, &next));
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
        .map(Box::new)
        .ok_or_else(|| {
            E::MissedExpectation(token.to_string(), "explicit type".into()).link_with_token(&token)
        })?;
        let assignation = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Statement(&[StatementId::AssignedValue])],
        )?
        .map(Box::new)
        .ok_or_else(|| {
            E::MissedExpectation(token.to_string(), "initializer".into()).link_with_token(&token)
        })?;
        Ok(Some(GlobalDeclaration {
            token: token.clone(),
            variable: Box::new(variable),
            r#type: ty,
            assignation,
            uuid: Uuid::new_v4(),
        }))
    }
}

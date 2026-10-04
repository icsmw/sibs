#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for EnvDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Keyword(Keyword::Required | Keyword::Optional)
        )
    }
}

impl ReadNode<EnvDeclaration> for EnvDeclaration {
    fn read(parser: &Parser) -> Result<Option<EnvDeclaration>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !Self::intrested(&token) {
            return Ok(None);
        }
        let variable = LinkedNode::try_read(
            parser,
            NodeTarget::Declaration(&[DeclarationId::VariableName]),
        )?
        .ok_or_else(|| E::MissedEnvName.link_with_token(&token))?;
        if !variable.get_md().ppm.is_empty() {
            return Err(E::UnrecognizedCode(variable.to_string()).link(&variable));
        }
        Ok(Some(EnvDeclaration {
            token: token.clone(),
            variable: Box::new(variable),
            uuid: Uuid::new_v4(),
        }))
    }
}

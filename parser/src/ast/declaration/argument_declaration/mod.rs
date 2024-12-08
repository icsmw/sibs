#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ArgumentDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..))
    }
}

impl ReadNode<ArgumentDeclaration> for ArgumentDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<ArgumentDeclaration>, LinkedErr<E>> {
        let Some(variable) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Declaration(&[DeclarationId::VariableName])],
        )?
        else {
            return Ok(None);
        };
        let Some(ty) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Declaration(&[
                DeclarationId::VariableTypeDeclaration,
                DeclarationId::VariableVariants,
            ])],
        )?
        .map(Box::new) else {
            return Err(E::MissedArgumentTypeDefinition.link(&variable));
        };
        Ok(Some(ArgumentDeclaration {
            variable: Box::new(variable),
            r#type: ty,
            uuid: Uuid::new_v4(),
        }))
    }
}

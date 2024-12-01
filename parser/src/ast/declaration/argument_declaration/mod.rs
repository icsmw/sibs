#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;

impl ReadNode<ArgumentDeclaration> for ArgumentDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<ArgumentDeclaration>, LinkedErr<E>> {
        let Some(variable) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[ExpressionId::Variable])],
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
            return Err(E::MissedArgumentTypeDefinition.link(&(&variable).into()));
        };
        Ok(Some(ArgumentDeclaration {
            variable: Box::new(variable),
            r#type: ty,
            uuid: Uuid::new_v4(),
        }))
    }
}

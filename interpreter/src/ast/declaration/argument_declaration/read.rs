use crate::*;

impl ReadNode<ArgumentDeclaration> for ArgumentDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<ArgumentDeclaration>, LinkedErr<E>> {
        let Some(variable) =
            Expression::try_read(parser, ExpressionId::Variable)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(ty) = Node::try_oneof(
            parser,
            &[NodeReadTarget::Declaration(&[
                DeclarationId::VariableTypeDeclaration,
            ])],
        )?
        .map(Box::new) else {
            return Err(E::MissedArgumentTypeDefinition.link(&(&variable).into()));
        };
        Ok(Some(ArgumentDeclaration {
            variable: Box::new(variable),
            r#type: ty,
        }))
    }
}

#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for VariableType {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Keyword(Keyword::Str)
                | Kind::Keyword(Keyword::Bool)
                | Kind::Keyword(Keyword::Num)
                | Kind::Keyword(Keyword::Vec)
        )
    }
}

impl ReadNode<VariableType> for VariableType {
    fn read(parser: &Parser) -> Result<Option<VariableType>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        match &token.kind {
            Kind::Keyword(Keyword::Bool)
            | Kind::Keyword(Keyword::Str)
            | Kind::Keyword(Keyword::Num) => Ok(Some(VariableType {
                r#type: VariableTypeDef::Primitive(token.clone()),
                uuid: Uuid::new_v4(),
            })),
            Kind::Keyword(Keyword::Vec) => {
                let (mut inner, ..) = parser
                    .between(KindId::Less, KindId::Greater)?
                    .ok_or_else(|| E::MissedVariableTypeDefinition.link_with_token(&token))?;
                let ty = LinkedNode::try_oneof(
                    &mut inner,
                    &[NodeTarget::Declaration(&[DeclarationId::VariableType])],
                )?
                .ok_or_else(|| E::MissedVariableTypeDefinition.link_with_token(&token))?;
                if !inner.is_done() {
                    return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
                }
                Ok(Some(VariableType {
                    r#type: VariableTypeDef::Compound(VariableCompoundType::Vec(
                        token.clone(),
                        Box::new(ty),
                    )),
                    uuid: Uuid::new_v4(),
                }))
            }
            _ => Ok(None),
        }
    }
}

use crate::*;
use lexer::{Kind, KindId};

impl ReadNode<VariableType> for VariableType {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<VariableType>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if let Kind::Identifier(ident) = &token.kind {
            if let Ok(ty) = VariablePrimitiveType::try_from(ident.to_owned()) {
                Ok(Some(VariableType {
                    r#type: VariableTypeDef::Primitive(ty),
                    token,
                }))
            } else if VariableCompoundType::is_valid_alias(ident) {
                let Some(mut inner) = parser.between(KindId::Less, KindId::Greater)? else {
                    return Err(E::MissedVariableTypeDefinition);
                };
                let ty = VariableType::read(&mut inner, &Nodes::empty())?
                    .ok_or(E::MissedVariableTypeDefinition)?;
                if !inner.is_done() {
                    return Err(E::UnrecognizedCode(inner.to_string()));
                }
                Ok(Some(VariableType {
                    r#type: VariableTypeDef::Compound(VariableCompoundType::new(
                        Node::Declaration(Declaration::VariableType(ty)),
                        ident,
                    )?),
                    token,
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

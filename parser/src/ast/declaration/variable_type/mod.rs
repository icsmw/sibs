mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<VariableType> for VariableType {
    fn read(parser: &mut Parser) -> Result<Option<VariableType>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        match &token.kind {
            Kind::Keyword(Keyword::Bool)
            | Kind::Keyword(Keyword::Str)
            | Kind::Keyword(Keyword::Num) => Ok(Some(VariableType {
                r#type: VariableTypeDef::Primitive(token),
            })),
            Kind::Keyword(Keyword::Vec) => {
                let Some((mut inner, ..)) = parser.between(KindId::Less, KindId::Greater)? else {
                    return Err(E::MissedVariableTypeDefinition.link_with_token(&token));
                };
                let ty = VariableType::read(&mut inner)?
                    .ok_or(E::MissedVariableTypeDefinition.link_with_token(&token))?;
                if !inner.is_done() {
                    return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
                }
                Ok(Some(VariableType {
                    r#type: VariableTypeDef::Compound(VariableCompoundType::Vec(
                        token,
                        Box::new(Node::Declaration(Declaration::VariableType(ty))),
                    )),
                }))
            }
            _ => Ok(None),
        }
    }
}

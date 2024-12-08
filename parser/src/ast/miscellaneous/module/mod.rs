#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Module {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Mod))
    }
}

impl ReadNode<Module> for Module {
    fn read(parser: &mut Parser) -> Result<Option<Module>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Mod)) {
            return Ok(None);
        }
        let node = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Value(&[ValueId::PrimitiveString])],
        )?
        .ok_or_else(|| E::MissedModulePath.link_with_token(&token))?;
        Ok(Some(Module {
            token,
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}

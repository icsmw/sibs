#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ModuleDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Mod))
    }
}

impl ReadNode<ModuleDeclaration> for ModuleDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<ModuleDeclaration>, LinkedErr<E>> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Mod)) {
            return Ok(None);
        }
        let Some(from) = parser.token().cloned() else {
            return Ok(None);
        };
        let Kind::Identifier(vl) = &from.kind else {
            return Ok(None);
        };
        if vl != "from" {
            return Ok(None);
        }
        let node = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Value(&[ValueId::PrimitiveString])],
        )?
        .ok_or_else(|| E::MissedModulePath.link_with_token(&sig))?;
        Ok(Some(ModuleDeclaration {
            sig,
            from,
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}

#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<Include> for Include {
    fn read(parser: &mut Parser) -> Result<Option<Include>, LinkedErr<E>> {
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
        Ok(Some(Include {
            token,
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}

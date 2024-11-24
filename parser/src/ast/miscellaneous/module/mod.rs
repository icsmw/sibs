#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<Module> for Module {
    fn read(parser: &mut Parser) -> Result<Option<Module>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Mod)) {
            return Ok(None);
        }
        let node = Value::try_read(parser, ValueId::PrimitiveString)?
            .map(Node::Value)
            .ok_or_else(|| E::MissedModulePath.link_with_token(&token))?;
        Ok(Some(Module {
            token,
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}

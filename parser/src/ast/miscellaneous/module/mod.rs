mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use lexer::{Keyword, Kind};

impl ReadNode<Module> for Module {
    fn read(parser: &mut Parser) -> Result<Option<Module>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Mod)) {
            return Ok(None);
        }
        let Some(node) = Value::try_read(parser, ValueId::PrimitiveString)?.map(Node::Value) else {
            return Err(E::MissedModulePath.link_with_token(&token));
        };
        Ok(Some(Module {
            token,
            node: Box::new(node),
        }))
    }
}

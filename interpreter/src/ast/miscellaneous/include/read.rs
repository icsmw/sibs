use lexer::{Keyword, Kind};

use crate::*;

impl ReadNode<Include> for Include {
    fn read(parser: &mut Parser) -> Result<Option<Include>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Mod)) {
            return Ok(None);
        }
        let Some(node) = Value::try_read(parser, ValueId::PrimitiveString)?.map(Node::Value) else {
            return Err(E::MissedModulePath);
        };
        Ok(Some(Include {
            token,
            node: Box::new(node),
        }))
    }
}

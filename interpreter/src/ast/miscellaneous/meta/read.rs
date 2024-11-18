use lexer::Kind;

use crate::*;

impl ReadNode<Meta> for Meta {
    fn read(parser: &mut Parser) -> Result<Option<Meta>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Meta(..)) {
            return Ok(None);
        }
        Ok(Some(Meta { token }))
    }
}

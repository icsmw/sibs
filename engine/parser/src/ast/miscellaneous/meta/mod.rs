#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Meta {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Meta(..))
    }
}

impl ReadNode<Meta> for Meta {
    fn read(parser: &Parser) -> Result<Option<Meta>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Meta(..)) {
            return Ok(None);
        }
        Ok(Some(Meta {
            token,
            uuid: Uuid::new_v4(),
        }))
    }
}

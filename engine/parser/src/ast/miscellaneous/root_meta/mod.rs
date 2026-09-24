#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for RootMeta {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::RootMeta(..))
    }
}

impl ReadNode<RootMeta> for RootMeta {
    fn read(parser: &Parser) -> Result<Option<RootMeta>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::RootMeta(..)) {
            return Ok(None);
        }
        Ok(Some(RootMeta {
            token: token.clone(),
            uuid: Uuid::new_v4(),
        }))
    }
}

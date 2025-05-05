#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Comment {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Comment(..))
    }
}

impl ReadNode<Comment> for Comment {
    fn read(parser: &Parser) -> Result<Option<Comment>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Comment(..)) {
            return Ok(None);
        }
        Ok(Some(Comment {
            token: token.clone(),
            uuid: Uuid::new_v4(),
        }))
    }
}

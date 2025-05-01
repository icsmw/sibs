#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Break {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Break))
    }
}

impl ReadNode<Break> for Break {
    fn read(parser: &Parser) -> Result<Option<Break>, LinkedErr<E>> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        if !matches!(tk.kind, Kind::Keyword(Keyword::Break)) {
            return Ok(None);
        }
        Ok(Some(Break {
            token: tk.to_owned(),
            target: None,
            uuid: Uuid::new_v4(),
        }))
    }
}

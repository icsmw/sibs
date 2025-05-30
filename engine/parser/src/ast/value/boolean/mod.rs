#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Boolean {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Keyword(Keyword::True) | Kind::Keyword(Keyword::False)
        )
    }
}

impl ReadNode<Boolean> for Boolean {
    fn read(parser: &Parser) -> Result<Option<Boolean>, LinkedErr<E>> {
        if let Some(tk) = parser.token() {
            let node = Boolean {
                inner: if matches!(tk.kind, Kind::Keyword(Keyword::True)) {
                    true
                } else if matches!(tk.kind, Kind::Keyword(Keyword::False)) {
                    false
                } else {
                    return Ok(None);
                },
                token: tk.clone(),
                uuid: Uuid::new_v4(),
            };
            return Ok(Some(node));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for PrimitiveString {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::String(..))
    }
}

impl ReadNode<PrimitiveString> for PrimitiveString {
    fn read(parser: &Parser) -> Result<Option<PrimitiveString>, LinkedErr<E>> {
        if let Some(tk) = parser.token() {
            let Kind::String(inner) = &tk.kind else {
                return Ok(None);
            };
            return Ok(Some(PrimitiveString {
                inner: inner.to_owned(),
                token: tk.clone(),
                uuid: Uuid::new_v4(),
            }));
        }
        Ok(None)
    }
}

use lexer::Kind;

use crate::*;

impl ReadNode<PrimitiveString> for PrimitiveString {
    fn read(parser: &mut Parser) -> Result<Option<PrimitiveString>, LinkedErr<E>> {
        if let Some(tk) = parser.token() {
            let Kind::String(inner) = &tk.kind else {
                return Ok(None);
            };
            return Ok(Some(PrimitiveString {
                inner: inner.to_owned(),
                token: tk.clone(),
            }));
        }
        Ok(None)
    }
}

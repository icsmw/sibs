use lexer::Kind;

use crate::*;

impl ReadNode<Number> for Number {
    fn read(parser: &mut Parser) -> Result<Option<Number>, E> {
        if let Some(tk) = parser.token() {
            let Kind::Number(inner) = &tk.kind else {
                return Ok(None);
            };
            if inner.is_infinite() {
                return Err(E::InfiniteNumber);
            }
            return Ok(Some(Number {
                inner: inner.to_owned(),
                token: tk.clone(),
            }));
        }
        Ok(None)
    }
}

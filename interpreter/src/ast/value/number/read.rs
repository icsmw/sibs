use lexer::Kind;

use crate::*;

impl ReadElement<Number> for Number {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Number>, E> {
        if let Some(tk) = parser.token() {
            let Kind::Number(inner) = &tk.kind else {
                return Ok(None);
            };
            if inner.is_infinite() {
                return Err(E::InfiniteNumber);
            }
            let node = Number {
                inner: inner.to_owned(),
                token: tk.clone(),
            };
            parser.advance();
            return Ok(Some(node));
        }
        Ok(None)
    }
}

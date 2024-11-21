mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use lexer::Kind;

impl ReadNode<Number> for Number {
    fn read(parser: &mut Parser) -> Result<Option<Number>, LinkedErr<E>> {
        if let Some(tk) = parser.token() {
            let Kind::Number(inner) = &tk.kind else {
                return Ok(None);
            };
            if inner.is_infinite() {
                return Err(E::InfiniteNumber.link_with_token(tk));
            }
            return Ok(Some(Number {
                inner: inner.to_owned(),
                token: tk.clone(),
            }));
        }
        Ok(None)
    }
}

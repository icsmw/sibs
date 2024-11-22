#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<Boolean> for Boolean {
    fn read(parser: &mut Parser) -> Result<Option<Boolean>, LinkedErr<E>> {
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
            };
            return Ok(Some(node));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<Break> for Break {
    fn read(parser: &mut Parser) -> Result<Option<Break>, LinkedErr<E>> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        if !matches!(tk.kind, Kind::Keyword(Keyword::Break)) {
            return Ok(None);
        }
        Ok(Some(Break {
            token: tk.to_owned(),
            uuid: Uuid::new_v4(),
        }))
    }
}

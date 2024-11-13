use lexer::{Keyword, Kind};

use crate::*;

impl ReadNode<Boolean> for Boolean {
    fn read(parser: &mut Parser) -> Result<Option<Boolean>, E> {
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

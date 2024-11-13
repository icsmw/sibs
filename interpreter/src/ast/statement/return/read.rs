use lexer::{Keyword, Kind};

use crate::*;

impl ReadNode<Return> for Return {
    fn read(parser: &mut Parser) -> Result<Option<Return>, E> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        if !matches!(tk.kind, Kind::Keyword(Keyword::Return)) {
            return Ok(None);
        }
        Ok(Some(Return {
            token: tk.to_owned(),
        }))
    }
}

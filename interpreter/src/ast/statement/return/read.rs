use lexer::KindId;

use crate::*;

impl ReadNode<Return> for Return {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Return>, E> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        if tk.id() != KindId::Return {
            return Ok(None);
        }
        Ok(Some(Return {
            token: tk.to_owned(),
        }))
    }
}

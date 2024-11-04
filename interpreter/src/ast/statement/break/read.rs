use lexer::KindId;

use crate::*;

impl ReadElement<Break> for Break {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Break>, E> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        if tk.id() != KindId::Break {
            return Ok(None);
        }
        let node = Some(Break {
            token: tk.to_owned(),
        });
        parser.advance();
        Ok(node)
    }
}

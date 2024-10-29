use lexer::KindId;

use crate::*;

impl ReadElement<Break> for Break {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Break>, E> {
        if let Some(tk) = parser.token() {
            if tk.id() == KindId::Break {
                parser.advance();
                return Ok(Some(Break {}));
            }
        }
        Ok(None)
    }
}

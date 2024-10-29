use lexer::KindId;

use crate::*;

impl ReadElement<Return> for Return {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Return>, E> {
        if let Some(tk) = parser.token() {
            if tk.id() == KindId::Return {
                parser.advance();
                return Ok(Some(Return {}));
            }
        }
        Ok(None)
    }
}

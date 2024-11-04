use lexer::Kind;

use crate::*;

impl ReadElement<Boolean> for Boolean {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Boolean>, E> {
        if let Some(tk) = parser.token() {
            let node = Boolean {
                inner: match tk.kind {
                    Kind::True => true,
                    Kind::False => false,
                    _ => return Ok(None),
                },
                token: tk.clone(),
            };
            parser.advance();
            return Ok(Some(node));
        }
        Ok(None)
    }
}

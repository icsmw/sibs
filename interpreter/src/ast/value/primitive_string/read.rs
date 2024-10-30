use lexer::Kind;

use crate::*;

impl ReadElement<PrimitiveString> for PrimitiveString {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<PrimitiveString>, E> {
        if let Some(tk) = parser.token() {
            let Kind::String(inner) = &tk.kind else {
                return Ok(None);
            };
            let node = PrimitiveString {
                inner: inner.to_owned(),
            };
            parser.advance();
            return Ok(Some(node));
        }
        Ok(None)
    }
}

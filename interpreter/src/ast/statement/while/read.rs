use lexer::Kind;

use crate::*;

impl ReadElement<While> for While {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<While>, E> {
        Ok(None)
    }
}

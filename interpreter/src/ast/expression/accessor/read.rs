use lexer::Kind;

use crate::*;

impl ReadElement<Accessor> for Accessor {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Accessor>, E> {
        Ok(None)
    }
}

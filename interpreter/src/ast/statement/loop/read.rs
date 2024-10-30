use lexer::Kind;

use crate::*;

impl ReadElement<Loop> for Loop {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Loop>, E> {
        Ok(None)
    }
}

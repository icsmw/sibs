use lexer::Kind;

use crate::*;

impl ReadElement<Call> for Call {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Call>, E> {
        Ok(None)
    }
}

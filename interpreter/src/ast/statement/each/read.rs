use lexer::Kind;

use crate::*;

impl ReadElement<Each> for Each {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Each>, E> {
        Ok(None)
    }
}

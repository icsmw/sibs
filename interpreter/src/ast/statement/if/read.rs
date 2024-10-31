use lexer::Kind;

use crate::*;

impl ReadElement<If> for If {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<If>, E> {
        Ok(None)
    }
}

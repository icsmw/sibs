use lexer::Kind;

use crate::*;

impl ReadElement<Task> for Task {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Task>, E> {
        Ok(None)
    }
}
